
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


use serde::{Deserialize, Serialize};

pub mod config;

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Template {
    pub name: String,
    pub version: [u8; 3]
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Skill {
    pub name: String,
    pub value: i64
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Attribute {
    pub name: String,
    pub value: i64,
    pub skills: Option<Vec<Skill>>
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CharacterSheet {
    pub name: String,
    pub template: Template,
    pub description: Option<String>,
    pub health: i64,
    pub armor_class: i64,
    pub weapon_proficiencies: Vec<String>,
    pub perks: Option<Vec<String>>,    
    pub attributes: Vec<Attribute>
}