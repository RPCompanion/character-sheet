
use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub description: String,
    pub skills: Vec<Skill>,
    pub required: Option<Requirements>
}