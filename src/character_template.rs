
use serde::{Deserialize, Serialize};

pub mod perk;
pub mod weapon_proficiency;
pub mod attributes;
pub mod common;

use attributes::Attribute;
use perk::Perk;
use weapon_proficiency::WeaponProficiency;

use crate::character_sheet::CharacterSheet;

/**
 * 
 * How many points a character has for a particular attribute/skill/perk
 * and how many points they can allocate to a single attribute/skill/perk
 * 
*/
#[derive(Serialize, Deserialize)]
pub struct Points {
    pub given_points: u64,
    pub max_points_per_allotment: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Allotment {
    pub attributes: Points,
    pub skills: Option<Points>,
    pub perks: Option<Points>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterTemplate {
    pub name: String,
    pub version: [u8; 3],
    pub description: String,

    pub base_health: i32,
    pub base_armor_class: i32,

    pub allotments: Allotment,
    pub weapon_proficiencies: Option<WeaponProficiency>,
    pub perks: Option<Vec<Perk>>,
    pub attributes: Vec<Attribute>,
}

impl CharacterTemplate {

    pub fn validate(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

        sheet.validate()?;

        if !self.valid_template_name(sheet) {
            return Err("Character template name mismatch");
        }

        if self.valid_version(sheet) {
            return Err("Character template version mismatch");
        }

        Ok(())

    }

    fn valid_template_name(&self, sheet: &CharacterSheet) -> bool {

        sheet.template.name == self.name

    }

    fn valid_version(&self, sheet: &CharacterSheet) -> bool {

        if sheet.template.version.len() != self.version.len() {
            return false;
        }

        self.version.iter().eq(sheet.template.version.iter())

    }

}