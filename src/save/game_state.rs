use eframe::epaint::ahash::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameState {
    #[serde_as(as = "JsonString")]
    game_state: GameStateInner,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameStateInner {
    game_type: String,

    game_days: i32,
    game_hours: i32,
    game_minutes: i32,
    game_seconds: i32,
    game_milliseconds: i32,

    is_robby_dead: bool,
    is_virginia_dead: bool,
    core_game_completed: bool,
    escaped_island: bool,
    stayed_on_island: bool,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}
