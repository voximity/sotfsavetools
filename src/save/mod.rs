mod game_state;
mod save_data;

use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

pub use game_state::*;
pub use save_data::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenericData<T> {
    version: String,
    data: T,
}

pub struct Save {
    game_state: GameState,
    save_data: SaveData,
}

impl Save {
    pub fn read(path: PathBuf) -> io::Result<Self> {
        macro_rules! load_file {
            ($name:ident : $type:ty => $file:literal) => {
                let $name: $type = {
                    let file = File::open(path.join($file))?;
                    serde_json::from_reader(BufReader::new(file))
                        .expect(concat!("failed to parse", stringify!($name)))
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
                let file = File::open(path.join($file))?;
                serde_json::to_writer(file, &self.$name)
                    .expect(concat!("failed to write", stringify!($name)));
            };};
        }

        write_file!(game_state => "GameStateSaveData.json");
        write_file!(save_data => "SaveData.json");

        Ok(())
    }
}
