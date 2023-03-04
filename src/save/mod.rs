mod game_state;
mod save_data;

use std::{
    fmt::Display,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

pub use game_state::*;
pub use save_data::*;

use serde::{Deserialize, Serialize};

use crate::tools::SaveTools;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenericData<T> {
    pub version: String,
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct Save {
    pub game_state: GenericData<GameState>,
    pub save_data: GenericData<SaveData>,
}

/// A save type.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SaveType {
    Singleplayer,
    Multiplayer,
}

impl Display for SaveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SaveType::Singleplayer => "Singleplayer",
            SaveType::Multiplayer => "Multiplayer",
        })
    }
}

impl SaveType {
    pub fn as_file(&self) -> &'static str {
        match self {
            Self::Singleplayer => "SinglePlayer",
            Self::Multiplayer => "Multiplayer",
        }
    }
}

pub type SelectedSave = (String, SaveType, String);

#[derive(Debug, Clone)]
pub struct SaveInstance {
    pub path: SelectedSave,
    pub save: Save,
    pub tools: SaveTools,
}

impl SaveInstance {
    pub fn new(path: SelectedSave, save: Save) -> Self {
        let tools = SaveTools::new(&save);
        Self { path, save, tools }
    }
}

macro_rules! get_type_id_methods {
    ($name:ident , $name_mut:ident : $type:ty => $($p:ident).*) => {
        pub fn $name(&self, type_id: u32) -> Option<&$type> {
            self.$($p.)+iter().find(|e| e.type_id == type_id)
        }

        pub fn $name_mut(&mut self, type_id: u32) -> Option<&mut $type> {
            self.$($p.)+iter_mut().find(|e| e.type_id == type_id)
        }
    };
}

impl Save {
    pub fn read(path: PathBuf) -> io::Result<Self> {
        macro_rules! load_file {
            ($name:ident : $type:ty => $file:literal) => {
                let $name: GenericData<$type> = {
                    let file = File::open(path.join($file))?;
                    serde_json::from_reader(BufReader::new(file))
                        .expect(concat!("failed to parse ", stringify!($name)))
                };
            };
        }

        load_file!(game_state: GameState => "GameStateSaveData.json");
        load_file!(save_data: SaveData => "SaveData.json");

        Ok(Self {
            game_state,
            save_data,
        })
    }

    pub fn write(&self, path: PathBuf) -> io::Result<()> {
        macro_rules! write_file {
            ($name:ident => $file:literal) => {{
                let file = File::create(path.join($file))?;
                serde_json::to_writer(file, &self.$name)
                    .expect(concat!("failed to write", stringify!($name)));
            };};
        }

        write_file!(game_state => "GameStateSaveData.json");
        write_file!(save_data => "SaveData.json");

        Ok(())
    }

    get_type_id_methods!(
        actor, actor_mut: Actor =>
            save_data.data.vail_world_sim.actors
    );

    get_type_id_methods!(
        kill_stat, kill_stat_mut: KillStat =>
            save_data.data.vail_world_sim.kill_stats_list
    );
}

mod f32_nan {
    use serde::{de::Visitor, Deserializer, Serializer};

    pub struct F32NanVisitor;

    impl<'de> Visitor<'de> for F32NanVisitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a float or \"NaN\"")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v == "NaN" {
                Ok(f32::NAN)
            } else {
                Err(E::custom(format!("non-NaN f32 string: {}", v)))
            }
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as f32)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as f32)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as f32)
        }

        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as f32)
        }
    }

    pub fn serialize<S: Serializer>(n: &f32, s: S) -> Result<S::Ok, S::Error> {
        if n.is_nan() {
            s.serialize_str("NaN")
        } else {
            s.serialize_f32(*n)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
        d.deserialize_any(F32NanVisitor)
    }
}
