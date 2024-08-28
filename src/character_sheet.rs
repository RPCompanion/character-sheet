
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use crate::InternalVersion;

pub mod config;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct SheetTemplate {
    pub name: String,
    pub version: InternalVersion
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct SheetSkill {
    pub name: String,
    pub value: i64
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct SheetAttribute {
    pub name: String,
    pub value: i64,
    pub skills: Option<Vec<SheetSkill>>
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct CharacterSheet {
    pub name: String,
    pub template: SheetTemplate,
    pub description: Option<String>,
    pub health: i64,
    pub armor_class: i64,
    pub weapon_proficiencies: Vec<String>,
    pub perks: Option<Vec<String>>,    
    pub attributes: Vec<SheetAttribute>
}