
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
    pub given_points: i64,
    pub max_points_per_allotment: Option<i64>,
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

    pub base_health: i64,
    pub base_armor_class: i64,

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
        self.valid_perks(sheet)?;
        self.valid_perk_allotment(sheet)?;
        self.valid_attributes(sheet)?;
        self.valid_attribute_allotment(sheet)?;

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

    fn valid_perks(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

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

        } else if self.perks.is_some() && sheet.perks.is_some() {

            let template_perks = self.perks.as_ref().unwrap();
            let sheet_perks    = sheet.perks.as_ref().unwrap();

            for perk in sheet_perks {

                if !template_perks.iter().any(|tp| tp.name == *perk) {
                    return Err("Character template does not allow this perk");
                }

            }


        }
        
        Ok(())

    }

    fn valid_perk_allotment(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

        if let Some(perk_points) = &self.allotments.perks {

            let template_perks = self.perks.as_ref().unwrap();
            let sheet_perks    = sheet.perks.as_ref().unwrap();

            let total_points: i64 = sheet_perks
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

    fn valid_attributes(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

        for attribute in sheet.attributes.iter() {

            if !self.attributes.iter().any(|ta| ta.name == attribute.name) {
                return Err("Character template does not allow this attribute");
            }

        }

        Ok(())

    }

    fn valid_attribute_allotment(&self, sheet: &CharacterSheet) -> Result<(), &'static str> {

        let t_attr_points = &self.allotments.attributes;

        let mut s_total_points: i64 = 0;
        if let Some(max_points_per_allotment) = t_attr_points.max_points_per_allotment {

            for s_attr in sheet.attributes.iter() {
                
                if s_attr.value > max_points_per_allotment {
                    return Err("Character template does not allow this many points for a single attribute");
                }

                s_total_points += s_attr.value;
            }

        } else {

            s_total_points = sheet.attributes
                .iter()
                .map(|a| a.value)
                .sum();

        }

        if s_total_points > t_attr_points.given_points {
            return Err("Character template does not allow enough attribute points");
        }

        Ok(())

    }

}