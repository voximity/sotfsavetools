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

    get_type_id_methods!(
        actor, actor_mut: Actor =>
            save_data.data.vail_world_sim.actors
    );

    get_type_id_methods!(
        kill_stat, kill_stat_mut: KillStat =>
            save_data.data.vail_world_sim.kill_stats_list
    );

    pub fn resurrect_virginia(&mut self) {
        // set game state flag
        self.game_state.data.game_state.is_virginia_dead = false;

        // find virginia's actor
        if let Some(virginia) = self.actor_mut(10) {
            virginia.state = 2;
            if let Some(stats) = &mut virginia.stats {
                stats.health = 120.0;
            }
        }

        // remove the player killed stat, if any
        if let Some(kill) = self.kill_stat_mut(10) {
            kill.player_killed = 0;
        }
    }
}
