#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct WeaponProficiency {
    pub categories: Vec<WeaponCategory>
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct WeaponCategory {
    pub category: String,
    pub weapons: Vec<Weapon>
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Weapon {
    pub weapon: String,
    pub required: Option<Requirements>
}