#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct WeaponProficiency {
    pub categories: Vec<WeaponCategory>
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct WeaponCategory {
    pub category: String,
    pub weapons: Vec<Weapon>
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct Weapon {
    pub weapon: String,
    pub required: Option<Requirements>
}