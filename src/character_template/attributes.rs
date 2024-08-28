#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Attribute {
    pub name: String,
    pub description: String,
    pub skills: Option<Vec<Skill>>,
    pub required: Option<Requirements>
}