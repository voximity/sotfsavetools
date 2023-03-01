use std::{fs, path::PathBuf, time::SystemTime};

use chrono::{DateTime, Local};
use save::Save;

mod save;

fn main() {
    eframe::run_native(
        "Sons Of The Forest Save Tools",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|cc| Box::new(SotfApp::new(cc))),
    )
    .unwrap();
}

#[derive(Debug, Clone, Copy)]
enum SaveType {
    Singleplayer,
    Multiplayer,
}

impl From<SaveType> for &'static str {
    fn from(value: SaveType) -> Self {
        match value {
            SaveType::Singleplayer => "SinglePlayer",
            SaveType::Multiplayer => "Multiplayer",
        }
    }
}

#[derive(Clone, Default)]
struct SotfApp {
    /// The save directory.
    save_dir: PathBuf,

    /// The currently selected steam ID.
    steam_id: Option<String>,

    /// The currently selected save type.
    save_type: Option<SaveType>,

    /// The currently selected save name.
    save_name: Option<String>,

    /// The list of saves to select.
    saves: Option<Vec<(String, SystemTime)>>,

    /// The list of possible steam IDs.
    steam_ids: Vec<String>,

    /// The current save in-memory.
    save: Option<Save>,
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

        Self {
            save_dir,
            steam_id: match steam_ids.len() {
                1 => Some(steam_ids.first().unwrap().to_owned()),
                _ => None,
            },
            steam_ids,
            ..Default::default()
        }
    }

    pub fn save_type_dir(&self) -> PathBuf {
        self.save_dir
            .join(self.steam_id.to_owned().unwrap())
            .join(<&'static str>::from(self.save_type.unwrap()))
    }

    pub fn fetch_saves(&mut self) {
        let mut saves = fs::read_dir(self.save_type_dir())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .map(|entry| {
                (
                    entry.file_name().into_string().unwrap(),
                    entry.metadata().unwrap().modified().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        saves.sort_by(|(_, a), (_, b)| b.cmp(a));

        self.saves = Some(saves);
    }

    pub fn read_save(&mut self) {
        self.save = Some(
            Save::read(self.save_type_dir().join(self.save_name.as_ref().unwrap()))
                .expect("unable to read save data"),
        );
    }

    pub fn render_save_selector(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Save selector");

            if self.steam_id.is_none() {
                ui.label("Multiple Steam IDs found. Select one.");
                ui.add_space(10.0);

                for (i, id) in self.steam_ids.iter().enumerate() {
                    if ui.button(id).clicked() {
                        self.steam_id = Some(id.to_owned());
                    }

                    if i == 0 {
                        ui.label("This ID has more recent saves; this might be what you are looking for.");
                    }

                    ui.add_space(10.0);
                }
            } else if self.save_type.is_none() {
                ui.label("Select between the two save types.");

                ui.horizontal(|ui| {
                    if ui.button("Singleplayer").clicked() {
                        self.save_type = Some(SaveType::Singleplayer);
                        self.fetch_saves();
                    }

                    if ui.button("Multiplayer").clicked() {
                        self.save_type = Some(SaveType::Multiplayer);
                        self.fetch_saves();
                    }
                });
            } else if self.save_name.is_none() {
                ui.label("Select a save.");

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (name, time) in self.saves.as_ref().unwrap().iter() {
                        if ui.button(name).clicked() {
                            self.save_name = Some(name.to_owned());
                            self.saves = None;
                            self.read_save();
                            break;
                        }

                        ui.label(format!("{}", DateTime::<Local>::from(time.to_owned()).format("%B %-d, %Y @ %-I:%M:%S %p")));
                        ui.add_space(10.0);
                    }
                });
            } else {
                ui.spinner();
            }
        });
    }
}

impl eframe::App for SotfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(save) = &self.save {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Save editor");
                egui::Grid::new("save_editor").show(ui, |ui| {
                    ui.label("Kelvin");
                    if ui
                        .add_enabled(false, egui::Button::new("Resurrect Kelvin"))
                        .clicked()
                    {
                        // ...
                    }
                    ui.end_row();
                    ui.label("Virginia");
                    ui.label("Virginia settings");
                    ui.end_row();
                });
            });
        } else {
            self.render_save_selector(ctx);
        }
    }
}
