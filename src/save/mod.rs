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
    version: String,
    data: T,
}

#[derive(Debug, Clone)]
pub struct Save {
    pub game_state: GenericData<GameState>,
    pub save_data: GenericData<SaveData>,
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

    pub fn actor(&self, type_id: u32) -> Option<&Actor> {
        self.save_data
            .data
            .vail_world_sim
            .actors
            .iter()
            .find(|a| a.type_id == type_id)
    }

    pub fn actor_mut(&mut self, type_id: u32) -> Option<&mut Actor> {
        self.save_data
            .data
            .vail_world_sim
            .actors
            .iter_mut()
            .find(|a| a.type_id == type_id)
    }

    pub fn kill_stat(&self, type_id: u32) -> Option<&KillStat> {
        self.save_data
            .data
            .vail_world_sim
            .kill_stats_list
            .iter()
            .find(|s| s.type_id == type_id)
    }

    pub fn kill_stat_mut(&mut self, type_id: u32) -> Option<&mut KillStat> {
        self.save_data
            .data
            .vail_world_sim
            .kill_stats_list
            .iter_mut()
            .find(|s| s.type_id == type_id)
    }

    pub fn is_kelvin_dead(&self) -> bool {
        // TODO: cache this, maybe turn it into a macro

        if self.game_state.data.game_state.is_robby_dead {
            return true;
        }

        if let Some(kelvin) = self.actor(9) {
            if kelvin.state == 6 {
                return true;
            }

            if let Some(ActorStats { health, .. }) = kelvin.stats {
                if health <= 0.0 {
                    return true;
                }
            }
        }

        if let Some(kill) = self.kill_stat(9) {
            return kill.player_killed != 0;
        }

        false
    }

    pub fn resurrect_kelvin(&mut self) {
        // set game state flag
        self.game_state.data.game_state.is_robby_dead = false;

        // find kelvin's actor
        if let Some(kelvin) = self.actor_mut(9) {
            kelvin.state = 2;
            if let Some(stats) = &mut kelvin.stats {
                stats.health = 100.0;
            }
        }

        // remove the player killed stat, if any
        if let Some(kill) = self.kill_stat_mut(9) {
            kill.player_killed = 0;
        }
    }

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
