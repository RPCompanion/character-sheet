#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct AttributeModifier {
    pub name: String,
    pub modifier: i32,
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct SkillModifier {
    pub name: String,
    pub modifier: i32,
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct AttributeRequirement {
    pub name: String,
    pub greater_than_or_equal_to: i32,
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct SkillRequirement {
    pub name: String,
    pub greater_than_or_equal_to: i32,
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Requirements {
    pub perks: Option<Vec<String>>,
    pub attributes: Option<Vec<AttributeRequirement>>,
    pub skills: Option<Vec<SkillRequirement>>
}