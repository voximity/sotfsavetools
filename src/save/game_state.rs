use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct GameState {
    #[serde_as(as = "JsonString")]
    pub game_state: GameStateInner,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct GameStateInner {
    pub game_type: String,

    pub game_days: i32,
    pub game_hours: i32,
    pub game_minutes: i32,
    pub game_seconds: i32,
    pub game_milliseconds: i32,

    pub is_robby_dead: bool,
    pub is_virginia_dead: bool,
    pub core_game_completed: bool,
    pub escaped_island: bool,
    pub stayed_on_island: bool,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}
