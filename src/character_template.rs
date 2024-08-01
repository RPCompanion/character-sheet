
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
    pub given_points: i32,
    pub max_points_per_allotment: Option<i32>,
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
        self.valid_template_name(sheet)?;
        self.valid_version(sheet)?;
        self.valid_perk_allotment(sheet)?;

        Ok(())

    }

    fn valid_template_name(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

        if sheet.template.name == self.name {
            return Ok(());
        }
        
        Err("Character template name mismatch")

    }

    fn valid_version(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

        if sheet.template.version.len() != self.version.len() {
            return Err("Character template version mismatch");
        }

        if self.version.iter().eq(sheet.template.version.iter()) {
            return Ok(());
        }

        Err("Character template version mismatch")

    }

    fn valid_perk_allotment(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

        if sheet.perks.is_none() {
            return Ok(());
        }

        if self.perks.is_none() && sheet.perks.is_none() {

            return Ok(());

        } else if self.perks.is_none() && sheet.perks.is_some() {

            if sheet.perks.as_ref().unwrap().is_empty() {
                return Ok(());
            } else {
                return Err("Character template does not allow perks");
            }

        }

        if let Some(perk_points) = &self.allotments.perks {

            let template_perks = self.perks.as_ref().unwrap();
            let sheet_perks    = sheet.perks.as_ref().unwrap();

            let total_points: i32 = sheet_perks
                .iter()
                .map(|p| {

                    template_perks
                        .iter()
                        .find(|tp| tp.name == *p)
                        .unwrap()
                        .point_cost

                })
                .sum();

            if total_points > perk_points.given_points {
                return Err("Character template does not allow enough perk points");
            }

        }

        Ok(())

    }

}