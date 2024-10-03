#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct AttributeModifier {
    pub name: String,
    pub modifier: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct SkillModifier {
    pub name: String,
    pub modifier: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct AttributeRequirement {
    pub name: String,
    pub greater_than_or_equal_to: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct SkillRequirement {
    pub name: String,
    pub greater_than_or_equal_to: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct Requirements {
    pub perks: Option<Vec<String>>,
    pub attributes: Option<Vec<AttributeRequirement>>,
    pub skills: Option<Vec<SkillRequirement>>
}