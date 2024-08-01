
use serde::{Deserialize, Serialize};

pub mod config;

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub version: [u8; 3]
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub value: i64
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: i64,
    pub skills: Option<Vec<Skill>>
}

#[derive(Serialize, Deserialize)]
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

impl CharacterSheet {

    pub fn validate(&self) -> Result<(), &'static str> {

        if let Some(description) = &self.description {

            if config::get_character_sheet_config().max_description_length >= (description.len() as i32) {
                return Err("Description too long");
            }

        }

        Ok(())

    }

}