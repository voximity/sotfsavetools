use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
    time::SystemTime,
};

use chrono::{DateTime, Local};
use save::{Save, SaveInstance, SaveType, SelectedSave};
use tools::SaveTool;

mod save;
mod tools;

fn main() {
    eframe::run_native(
        "Sons Of The Forest Save Tools",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(SotfApp::new(cc))),
    )
    .unwrap();
}

/// An asynchronous resource. Effectively an Option<T> with an extra
/// Loading variant.
#[derive(Debug, Clone, Default)]
enum AsyncOption<T> {
    #[default]
    None,
    Loading,
    Some(T),
}

impl<T> From<AsyncOption<T>> for Option<T> {
    fn from(value: AsyncOption<T>) -> Self {
        match value {
            AsyncOption::Some(v) => Some(v),
            _ => None,
        }
    }
}

/// All associated saves for a Steam ID.
#[derive(Default, Clone)]
struct SteamIdSaves {
    /// The Steam ID corresponding to the child saves.
    id: String,

    /// The saves, categorized by save type (singleplayer/multiplayer).
    saves: HashMap<SaveType, Vec<(String, SystemTime)>>,
}

/// The egui app.
#[derive(Clone, Default)]
struct SotfApp {
    /// The save directory.
    save_dir: PathBuf,

    saves: Vec<SteamIdSaves>,

    /// The current save in-memory.
    save: Arc<RwLock<AsyncOption<SaveInstance>>>,
}

impl SotfApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let save_dir = PathBuf::from(std::env::var("UserProfile").unwrap())
            .join("AppData\\LocalLow\\Endnight\\SonsOfTheForest\\Saves");

        assert!(
            save_dir.is_dir(),
            "unable to find Sons Of The Forest save data"
        );

        let steam_ids: Vec<String> = {
            // get all entries in the saves folder
            let mut save_dirs = fs::read_dir(save_dir.clone())
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().unwrap().is_dir())
                .map(|entry| {
                    (
                        entry.file_name(),
                        entry.metadata().unwrap().modified().unwrap(),
                    )
                })
                .collect::<Vec<_>>();

            // sort by modified date
            save_dirs.sort_by(|(_, a), (_, b)| b.cmp(a));

            // map to a list of names
            save_dirs
                .into_iter()
                .map(|(name, _)| name.into_string().unwrap())
                .collect()
        };

        assert!(
            !steam_ids.is_empty(),
            "Sons Of The Forest game data exists, but there is no save data"
        );

        let steam_id_saves = steam_ids
            .into_iter()
            .map(|id| {
                let mut saves = HashMap::new();

                for save_type in [SaveType::Singleplayer, SaveType::Multiplayer] {
                    let type_path = save_dir.join(&id).join(save_type.as_file());
                    let mut type_saves = vec![];

                    if !type_path.exists() {
                        continue;
                    }

                    // grab all the save folders
                    type_saves.extend(
                        fs::read_dir(type_path)
                            .unwrap()
                            .filter_map(Result::ok)
                            .filter(|e| e.file_type().unwrap().is_dir())
                            .map(|e| {
                                (
                                    e.file_name().into_string().unwrap(),
                                    e.metadata().unwrap().modified().unwrap(),
                                )
                            }),
                    );

                    type_saves.sort_by(|(_, a), (_, b)| b.cmp(a));

                    saves.insert(save_type, type_saves);
                }

                SteamIdSaves { id, saves }
            })
            .collect::<Vec<_>>();

        Self {
            save_dir,
            saves: steam_id_saves,
            ..Default::default()
        }
    }

    /// The save path for a particular selected save.
    pub fn save_path(&self, (id, save_type, name): &SelectedSave) -> Option<PathBuf> {
        Some(self.save_dir.join(id).join(save_type.as_file()).join(name))
    }

    /// Read the save on another thread, updating the save mutex.
    pub fn read_save_async(&self, selected: SelectedSave) {
        let mutex = Arc::clone(&self.save);
        let save_path = match self.save_path(&selected) {
            Some(s) => s,
            None => return,
        };

        // the lock should drop immediately after this statement
        *mutex.write().unwrap() = AsyncOption::Loading;

        thread::spawn(move || {
            let save = Save::read(save_path).expect("failed to read save");

            let mut lock = mutex.write().unwrap();
            *lock = AsyncOption::Some(SaveInstance::new(selected, save));
        });
    }

    /// Write the save on another thread.
    pub fn write_save_async(&self, selected: SelectedSave) {
        // TODO: it would be nice if this did not hang the main thread
        // TODO: it does because the save editor takes a write lock
        // TODO: in other words, make the save editor *not* take a write lock
        // TODO: and let the individual save tools do the locking on their own

        let mutex = Arc::clone(&self.save);
        let save_path = match self.save_path(&selected) {
            Some(s) => s,
            None => return,
        };

        thread::spawn(move || {
            let lock = mutex.read().unwrap();
            if let AsyncOption::Some(ref instance) = *lock {
                instance
                    .save
                    .write(save_path)
                    .expect("failed to write save");
            }
        });
    }
}

macro_rules! format_time {
    ($var:expr) => {
        format!(
            "{}",
            DateTime::<Local>::from($var).format("%B %-d, %Y @ %-I:%M:%S %p")
        )
    };
}

impl eframe::App for SotfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let selected_save = {
            match *self.save.read().unwrap() {
                AsyncOption::Some(SaveInstance { ref path, .. }) => Some(path.to_owned()),
                _ => None,
            }
        };

        egui::SidePanel::left("panel_save_selector").show(ctx, |ui| {
            ui.heading("Save selector");
            ui.label("Select a save below.");

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for id_saves in self.saves.iter() {
                        egui::CollapsingHeader::new(&id_saves.id)
                            .default_open(self.saves.len() == 1)
                            .show(ui, |ui| {
                                for (save_type, saves) in id_saves.saves.iter() {
                                    ui.collapsing(format!("{}", save_type), |ui| {
                                        for (name, time) in saves.iter() {
                                            if ui
                                                .add_enabled(
                                                    selected_save
                                                        .as_ref()
                                                        .map_or(true, |(_, _, sel_name)| {
                                                            sel_name != name
                                                        }),
                                                    egui::Button::new(name),
                                                )
                                                .on_hover_text(format_time!(time.to_owned()))
                                                .clicked()
                                            {
                                                self.read_save_async((
                                                    id_saves.id.to_owned(),
                                                    *save_type,
                                                    name.to_owned(),
                                                ));
                                            }
                                        }
                                    });
                                }
                            });
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mutex = Arc::clone(&self.save);

            // TODO: this should not take a write lock
            // TODO: let individual save tools take write locks
            let mut lock = mutex.write().unwrap();

            match *lock {
                AsyncOption::None => {
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.heading("Please select a save.");
                        },
                    );
                }
                AsyncOption::Loading => {
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        egui::Ui::spinner,
                    );
                }
                AsyncOption::Some(ref mut save) => {
                    ui.heading("Save editor");

                    egui::Grid::new("save_editor")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Kelvin");
                            save.tools.kelvin.render(&mut save.save, ui);
                            ui.end_row();

                            ui.label("Virginia");
                            save.tools.virginia.render(&mut save.save, ui);
                            ui.end_row();

                            ui.label("Save");
                            if ui.button("Save changes").clicked() {
                                self.write_save_async(selected_save.unwrap());
                            }
                            ui.end_row();
                        });
                }
            }
        });
    }
}
