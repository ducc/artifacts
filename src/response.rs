use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fmt;
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse<T> {
    pub data: Option<T>,
    pub error: Option<ErrorData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for ErrorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionData {
    pub cooldown: Cooldown,
    pub destination: Option<Destination>,
    pub character: Character,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub total_seconds: i64,
    pub remaining_seconds: i64,
    pub expiration: String,
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
    pub name: String,
    pub x: i64,
    pub y: i64,
    pub content: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub name: String,
    pub skin: String,
    pub level: i64,
    pub xp: i64,
    #[serde(rename = "max_xp")]
    pub max_xp: i64,
    #[serde(rename = "total_xp")]
    pub total_xp: i64,
    pub gold: i64,
    pub speed: i64,
    #[serde(rename = "mining_level")]
    pub mining_level: i64,
    #[serde(rename = "mining_xp")]
    pub mining_xp: i64,
    #[serde(rename = "mining_max_xp")]
    pub mining_max_xp: i64,
    #[serde(rename = "woodcutting_level")]
    pub woodcutting_level: i64,
    #[serde(rename = "woodcutting_xp")]
    pub woodcutting_xp: i64,
    #[serde(rename = "woodcutting_max_xp")]
    pub woodcutting_max_xp: i64,
    #[serde(rename = "fishing_level")]
    pub fishing_level: i64,
    #[serde(rename = "fishing_xp")]
    pub fishing_xp: i64,
    #[serde(rename = "fishing_max_xp")]
    pub fishing_max_xp: i64,
    #[serde(rename = "weaponcrafting_level")]
    pub weaponcrafting_level: i64,
    #[serde(rename = "weaponcrafting_xp")]
    pub weaponcrafting_xp: i64,
    #[serde(rename = "weaponcrafting_max_xp")]
    pub weaponcrafting_max_xp: i64,
    #[serde(rename = "gearcrafting_level")]
    pub gearcrafting_level: i64,
    #[serde(rename = "gearcrafting_xp")]
    pub gearcrafting_xp: i64,
    #[serde(rename = "gearcrafting_max_xp")]
    pub gearcrafting_max_xp: i64,
    #[serde(rename = "jewelrycrafting_level")]
    pub jewelrycrafting_level: i64,
    #[serde(rename = "jewelrycrafting_xp")]
    pub jewelrycrafting_xp: i64,
    #[serde(rename = "jewelrycrafting_max_xp")]
    pub jewelrycrafting_max_xp: i64,
    #[serde(rename = "cooking_level")]
    pub cooking_level: i64,
    #[serde(rename = "cooking_xp")]
    pub cooking_xp: i64,
    #[serde(rename = "cooking_max_xp")]
    pub cooking_max_xp: i64,
    pub hp: i64,
    pub haste: i64,
    #[serde(rename = "critical_strike")]
    pub critical_strike: i64,
    pub stamina: i64,
    #[serde(rename = "attack_fire")]
    pub attack_fire: i64,
    #[serde(rename = "attack_earth")]
    pub attack_earth: i64,
    #[serde(rename = "attack_water")]
    pub attack_water: i64,
    #[serde(rename = "attack_air")]
    pub attack_air: i64,
    #[serde(rename = "dmg_fire")]
    pub dmg_fire: i64,
    #[serde(rename = "dmg_earth")]
    pub dmg_earth: i64,
    #[serde(rename = "dmg_water")]
    pub dmg_water: i64,
    #[serde(rename = "dmg_air")]
    pub dmg_air: i64,
    #[serde(rename = "res_fire")]
    pub res_fire: i64,
    #[serde(rename = "res_earth")]
    pub res_earth: i64,
    #[serde(rename = "res_water")]
    pub res_water: i64,
    #[serde(rename = "res_air")]
    pub res_air: i64,
    pub x: i64,
    pub y: i64,
    pub cooldown: i64,
    #[serde(rename = "cooldown_expiration")]
    pub cooldown_expiration: String,
    #[serde(rename = "weapon_slot")]
    pub weapon_slot: String,
    #[serde(rename = "shield_slot")]
    pub shield_slot: String,
    #[serde(rename = "helmet_slot")]
    pub helmet_slot: String,
    #[serde(rename = "body_armor_slot")]
    pub body_armor_slot: String,
    #[serde(rename = "leg_armor_slot")]
    pub leg_armor_slot: String,
    #[serde(rename = "boots_slot")]
    pub boots_slot: String,
    #[serde(rename = "ring1_slot")]
    pub ring1_slot: String,
    #[serde(rename = "ring2_slot")]
    pub ring2_slot: String,
    #[serde(rename = "amulet_slot")]
    pub amulet_slot: String,
    #[serde(rename = "artifact1_slot")]
    pub artifact1_slot: String,
    #[serde(rename = "artifact2_slot")]
    pub artifact2_slot: String,
    #[serde(rename = "artifact3_slot")]
    pub artifact3_slot: String,
    #[serde(rename = "consumable1_slot")]
    pub consumable1_slot: String,
    #[serde(rename = "consumable1_slot_quantity")]
    pub consumable1_slot_quantity: i64,
    #[serde(rename = "consumable2_slot")]
    pub consumable2_slot: String,
    #[serde(rename = "consumable2_slot_quantity")]
    pub consumable2_slot_quantity: i64,
    #[serde(rename = "inventory_slot1")]
    pub inventory_slot1: String,
    #[serde(rename = "inventory_slot1_quantity")]
    pub inventory_slot1_quantity: i64,
    #[serde(rename = "inventory_slot2")]
    pub inventory_slot2: String,
    #[serde(rename = "inventory_slot2_quantity")]
    pub inventory_slot2_quantity: i64,
    #[serde(rename = "inventory_slot3")]
    pub inventory_slot3: String,
    #[serde(rename = "inventory_slot3_quantity")]
    pub inventory_slot3_quantity: i64,
    #[serde(rename = "inventory_slot4")]
    pub inventory_slot4: String,
    #[serde(rename = "inventory_slot4_quantity")]
    pub inventory_slot4_quantity: i64,
    #[serde(rename = "inventory_slot5")]
    pub inventory_slot5: String,
    #[serde(rename = "inventory_slot5_quantity")]
    pub inventory_slot5_quantity: i64,
    #[serde(rename = "inventory_slot6")]
    pub inventory_slot6: String,
    #[serde(rename = "inventory_slot6_quantity")]
    pub inventory_slot6_quantity: i64,
    #[serde(rename = "inventory_slot7")]
    pub inventory_slot7: String,
    #[serde(rename = "inventory_slot7_quantity")]
    pub inventory_slot7_quantity: i64,
    #[serde(rename = "inventory_slot8")]
    pub inventory_slot8: String,
    #[serde(rename = "inventory_slot8_quantity")]
    pub inventory_slot8_quantity: i64,
    #[serde(rename = "inventory_slot9")]
    pub inventory_slot9: String,
    #[serde(rename = "inventory_slot9_quantity")]
    pub inventory_slot9_quantity: i64,
    #[serde(rename = "inventory_slot10")]
    pub inventory_slot10: String,
    #[serde(rename = "inventory_slot10_quantity")]
    pub inventory_slot10_quantity: i64,
    #[serde(rename = "inventory_slot11")]
    pub inventory_slot11: String,
    #[serde(rename = "inventory_slot11_quantity")]
    pub inventory_slot11_quantity: i64,
    #[serde(rename = "inventory_slot12")]
    pub inventory_slot12: String,
    #[serde(rename = "inventory_slot12_quantity")]
    pub inventory_slot12_quantity: i64,
    #[serde(rename = "inventory_slot13")]
    pub inventory_slot13: String,
    #[serde(rename = "inventory_slot13_quantity")]
    pub inventory_slot13_quantity: i64,
    #[serde(rename = "inventory_slot14")]
    pub inventory_slot14: String,
    #[serde(rename = "inventory_slot14_quantity")]
    pub inventory_slot14_quantity: i64,
    #[serde(rename = "inventory_slot15")]
    pub inventory_slot15: String,
    #[serde(rename = "inventory_slot15_quantity")]
    pub inventory_slot15_quantity: i64,
    #[serde(rename = "inventory_slot16")]
    pub inventory_slot16: String,
    #[serde(rename = "inventory_slot16_quantity")]
    pub inventory_slot16_quantity: i64,
    #[serde(rename = "inventory_slot17")]
    pub inventory_slot17: String,
    #[serde(rename = "inventory_slot17_quantity")]
    pub inventory_slot17_quantity: i64,
    #[serde(rename = "inventory_slot18")]
    pub inventory_slot18: String,
    #[serde(rename = "inventory_slot18_quantity")]
    pub inventory_slot18_quantity: i64,
    #[serde(rename = "inventory_slot19")]
    pub inventory_slot19: String,
    #[serde(rename = "inventory_slot19_quantity")]
    pub inventory_slot19_quantity: i64,
    #[serde(rename = "inventory_slot20")]
    pub inventory_slot20: String,
    #[serde(rename = "inventory_slot20_quantity")]
    pub inventory_slot20_quantity: i64,
    #[serde(rename = "inventory_max_items")]
    pub inventory_max_items: i64,
    pub task: String,
    #[serde(rename = "task_type")]
    pub task_type: String,
    #[serde(rename = "task_progress")]
    pub task_progress: i64,
    #[serde(rename = "task_total")]
    pub task_total: i64,
}

pub struct InventorySlot {
    pub name: String,
    pub code: String,
    pub quantity: i64,
}
