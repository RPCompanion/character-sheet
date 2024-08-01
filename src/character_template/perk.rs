
use serde::{Deserialize, Serialize};

use super::common::{AttributeModifier, SkillModifier};

#[derive(Serialize, Deserialize)]
pub struct Perk {
    pub name: String,
    pub description: String,
    pub point_cost: i32,
    pub attributes: Option<Vec<AttributeModifier>>,
    pub skills: Option<Vec<SkillModifier>>,
}