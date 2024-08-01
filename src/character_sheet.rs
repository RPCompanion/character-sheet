
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub version: [u8; 3]
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub value: i32
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: i32,
    pub skills: Option<Vec<Skill>>
}

#[derive(Serialize, Deserialize)]
pub struct CharacterSheet {
    pub name: String,
    pub template: Template,
    pub description: Option<String>,
    pub health: i32,
    pub armor_class: i32,
    pub weapon_proficiencies: Vec<String>,
    pub perks: Option<Vec<String>>,    
    pub attributes: Vec<Attribute>
}