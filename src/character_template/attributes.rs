#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct Attribute {
    pub name: String,
    pub description: String,
    pub skills: Option<Vec<Skill>>,
    pub required: Option<Requirements>
}