
use serde::{Deserialize, Serialize};

pub mod perk;
pub mod weapon_proficiency;
pub mod attributes;
pub mod common;

use attributes::Attribute;
use perk::Perk;
use weapon_proficiency::WeaponProficiency;

/**
 * 
 * How many points a character has for a particular attribute/skill/perk
 * and how many points they can allocate to a single attribute/skill/perk
 * 
*/
#[derive(Serialize, Deserialize)]
pub struct Points {
    pub given_points: u64,
    pub max_points_per_allotment: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Allotment {
    pub attributes: Points,
    pub skills: Option<Points>,
    pub perks: Option<Points>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterTemplate {
    pub name: String,
    pub version: [u8; 3],
    pub description: String,

    pub base_health: i32,
    pub base_armor_class: i32,

    pub allotments: Allotment,
    pub weapon_proficiencies: Option<WeaponProficiency>,
    pub perks: Option<Vec<Perk>>,
    pub attributes: Vec<Attribute>,
}