use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaveData {
    #[serde_as(as = "JsonString")]
    vail_world_sim: VailWorldSim,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VailWorldSim {
    actors: Vec<Actor>,
    kill_stats_list: Vec<KillStat>,
    player_stats: PlayerStats,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Actor {
    type_id: u32,
    state: u32,
    stats: ActorStats,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActorStats {
    health: f32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KillStat {
    type_id: u32,
    player_killed: i32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerStats {
    cut_trees: i32,
    seen_in_village_count: i32,
    last_sighted_time_hours: f32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}
