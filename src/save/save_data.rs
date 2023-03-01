use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaveData {
    #[serde_as(as = "JsonString")]
    pub vail_world_sim: VailWorldSim,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VailWorldSim {
    pub actors: Vec<Actor>,
    pub kill_stats_list: Vec<KillStat>,
    pub player_stats: PlayerStats,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Actor {
    pub type_id: u32,
    pub state: u32,
    pub stats: Option<ActorStats>,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActorStats {
    pub health: f32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KillStat {
    pub type_id: u32,
    pub player_killed: i32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerStats {
    pub cut_trees: i32,
    pub seen_in_village_count: i32,
    pub last_sighted_time_hours: f32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}
