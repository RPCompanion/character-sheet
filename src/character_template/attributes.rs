#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Attribute {
    pub name: String,
    pub description: String,
    pub skills: Option<Vec<Skill>>,
    pub required: Option<Requirements>
}