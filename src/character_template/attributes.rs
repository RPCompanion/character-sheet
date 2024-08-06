
use serde::{Deserialize, Serialize};

use super::common::Requirements;

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub name: String,
    pub description: String,
    pub skills: Option<Vec<Skill>>,
    pub required: Option<Requirements>
}