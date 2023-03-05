use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct PlayerInventory {
    #[serde_as(as = "JsonString")]
    pub player_inventory: PlayerInventoryInner,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct PlayerInventoryInner {
    // EquippedItems: currently held item
    // QuickSelect: an array of items and their hotkeys
    pub item_instance_manager_data: ItemInstanceManagerData,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct ItemInstanceManagerData {
    pub item_blocks: Vec<ItemBlock>,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemBlock {
    pub item_id: ItemId,
    pub total_count: i32,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

macro_rules! item_ids {
    {$($name:literal = $value:literal),*,} => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ItemId(u16);

        impl Display for ItemId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.0 {
                    $($value => f.write_str($name),)*
                    x => write!(f, "Unknown Item (ID {})", x),
                }
            }
        }
    }
}

item_ids! {
    "Alcohol" = 414,
    "Aloe Vera" = 451,
    "Air Canister" = 469,
    "Backpack" = 402,
    "Battery" = 527,
    "Blueprint Book" = 552,
    "Bone Armor" = 494,
    "Buckshot (Shotgun Ammo)" = 364,
    "Canned Food" = 434,
    "Cash" = 496,
    "Chainsaw" = 394,
    "Cloth" = 415,
    "Cooking Pot" = 517,
    "Crafted Spear" = 474,
    "Creepy Armor" = 593,
    "Cross" = 468,
    "Crossbow" = 365,
    "Crossbow Bolt" = 368,
    "Duct Tape" = 419,
    "Emergency Pack" = 483,
    "Energy Drink" = 439,
    "Energy Bar" = 441,
    "Energy Mix" = 461,
    "Energy Mix +" = 462,
    "Feather" = 479,
    "Fish" = 436,
    "Flare" = 440,
    "Flashlight" = 471,
    "Flask" = 426,
    "Food Tray" = 512,
    "Frag Grenade" = 381,
    "Golden Armor" = 572,
    "GPS Locator" = 529,
    "GPS Tracker" = 412,
    "Grab Bag" = 351,
    "Grappling Hook" = 560,
    "Guest Keycard" = 526,
    "Guide Book" = 589,
    "Health Mix" = 455,
    "Health Mix +" = 456,
    "Hide Armor" = 519,
    "Knife" = 380,
    "Leaf" = 484,
    "Leaf Armor" = 473,
    "Log" = 78,
    "Modern Arrow" = 373,
    "Molotovs" = 388,
    "MRE" = 438,
    "Noodles" = 421,
    "Pistol" = 355,
    "Pistol Ammo" = 362,
    "Pistol Silencer" = 374,
    "Plasma Lighter" = 413,
    "Printer Arrow" = 618,
    "Printer Resin" = 390,
    "Radio" = 590,
    "Raw Meat" = 433,
    "Rebreather" = 444,
    "Revolver" = 386,
    "Rock" = 393,
    "Rope" = 403,
    "Rope Gun" = 522,
    "Severed Arm" = 480,
    "Severed Leg" = 481,
    "Shotgun" = 358,
    "Skin Pouch" = 508,
    "Skull" = 430,
    "Sled" = 428,
    "Slug (Shotgun Ammo)" = 363,
    "Small Rock" = 476,
    "Stick" = 392,
    "Stone Arrow" = 507,
    "Stun Gun" = 353,
    "Stun Gun Ammo" = 369,
    "Swimsuit" = 619,
    "Tactical Axe" = 379,
    "Tarp" = 504,
    "Tech Armor" = 554,
    "Tech Mesh" = 553,
    "Torch" = 503,
    "Turtle Shell" = 506,
    "Walkie Talkie" = 486,
    "Wristwatch" = 410,
    "Zipline Rope" = 523,
}
