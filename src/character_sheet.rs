
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};

use crate::{InteralString, InternalVersion};

pub mod config;

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Template {
    pub name: InteralString,
    pub version: InternalVersion
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Skill {
    pub name: InteralString,
    pub value: i64
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Attribute {
    pub name: InteralString,
    pub value: i64,
    pub skills: Option<Vec<Skill>>
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CharacterSheet {
    pub name: InteralString,
    pub template: Template,
    pub description: Option<InteralString>,
    pub health: i64,
    pub armor_class: i64,
    pub weapon_proficiencies: Vec<InteralString>,
    pub perks: Option<Vec<InteralString>>,    
    pub attributes: Vec<Attribute>
}