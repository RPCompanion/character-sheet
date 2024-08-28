#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};

use crate::InteralString;

use super::common::{AttributeModifier, SkillModifier};

#[derive(Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Perk {
    pub name: InteralString,
    pub description: InteralString,
    pub point_cost: i64,
    pub attributes: Option<Vec<AttributeModifier>>,
    pub skills: Option<Vec<SkillModifier>>,
    pub base_health_modifier: Option<i64>,
    pub base_armor_class_modifier: Option<i64>
}