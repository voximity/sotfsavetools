mod game_state;
mod save_data;

use std::path::PathBuf;

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
    pub fn read(path: PathBuf) {}

    pub fn write(&self, path: PathBuf) {}
}
