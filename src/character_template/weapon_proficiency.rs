
use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Serialize, Deserialize)]
pub struct WeaponProficiency {
    pub categories: Vec<WeaponCategory>
}

#[derive(Serialize, Deserialize)]
pub struct WeaponCategory {
    pub category: String,
    pub weapons: Vec<Weapon>
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub weapon: String,
    pub required: Option<Requirements>
}