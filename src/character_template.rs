
use serde::{Deserialize, Serialize};

pub mod perk;
pub mod weapon_proficiency;
pub mod attributes;
pub mod common;

use attributes::Attribute;
use perk::Perk;
use weapon_proficiency::WeaponProficiency;

use crate::character_sheet::{CharacterSheet, CharacterSheetError};

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

pub enum CharacterTemplateError {

}

impl CharacterTemplate {

    pub fn validate(&self) -> Result<(), CharacterTemplateError> {
        Ok(())
    }

    pub fn validate_sheet(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        sheet.validate()?;
        self.valid_template_name(sheet)?;
        self.valid_version(sheet)?;
        self.valid_perks(sheet)?;
        self.valid_perk_allotment(sheet)?;
        self.valid_attributes(sheet)?;
        self.valid_attribute_allotment(sheet)?;
        self.valid_skills(sheet)?;
        self.valid_skill_allotment(sheet)?;

        Ok(())

    }

    fn valid_template_name(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        if sheet.template.name == self.name {
            return Ok(());
        }
        
        Err(CharacterSheetError::NameMismatch)

    }

    fn valid_version(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        if sheet.template.version.len() != self.version.len() {
            return Err(CharacterSheetError::VersionMismatch);
        }

        if self.version.iter().eq(sheet.template.version.iter()) {
            return Ok(());
        }

        Err(CharacterSheetError::VersionMismatch)

    }

    fn valid_perks(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        if sheet.perks.is_none() {
            return Ok(());
        }

        if self.perks.is_none() && sheet.perks.is_none() {

            return Ok(());

        } else if self.perks.is_none() && sheet.perks.is_some() {

            if sheet.perks.as_ref().unwrap().is_empty() {
                return Ok(());
            } else {
                return Err(CharacterSheetError::PerksNotAllowed);
            }

        } else if self.perks.is_some() && sheet.perks.is_some() {

            let template_perks = self.perks.as_ref().unwrap();
            let sheet_perks    = sheet.perks.as_ref().unwrap();

            for perk in sheet_perks {

                if !template_perks.iter().any(|tp| tp.name == *perk) {
                    return Err(CharacterSheetError::PerkNotAllowed(perk.clone()));
                }

            }


        }
        
        Ok(())

    }

    fn valid_perk_allotment(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

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
                return Err(CharacterSheetError::NotEnoughPerkPoints(total_points));
            }

        }

        Ok(())

    }

    fn valid_attributes(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        for attribute in sheet.attributes.iter() {

            if !self.attributes.iter().any(|ta| ta.name == attribute.name) {
                return Err(CharacterSheetError::AttributeNotAllowed(attribute.name.clone()));
            }

        }

        Ok(())

    }

    fn valid_attribute_allotment(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        let t_attr_points = &self.allotments.attributes;
        let max_points_per_allotment =  t_attr_points.max_points_per_allotment.unwrap_or(i64::MAX);

        let s_total_points = sheet.attributes
            .iter()
            .try_fold(0i64, |acc, attr| {

                if attr.value < 0 {

                    return Err(CharacterSheetError::NegativeAttributePoints {
                        offending_attribute: attr.name.clone(),
                        points: attr.value,
                    });

                }

                if attr.value > max_points_per_allotment {

                    return Err(CharacterSheetError::TooManyAttributePoints {
                        attribute: attr.name.clone(),
                        allotted_points: attr.value,
                        max_points: max_points_per_allotment,
                    });

                }

                Ok(acc + attr.value)

            })?;

        if s_total_points > t_attr_points.given_points {
            return Err(CharacterSheetError::AttributePointsExceeded(s_total_points));
        }

        Ok(())

    }

    fn valid_skills(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        for attribute in sheet.attributes.iter() {

            let template_attribute = self.attributes
                .iter()
                .find(|ta| ta.name == attribute.name)
                .unwrap();

            let sheet_skills = attribute.skills.as_ref();
            if sheet_skills.is_none() {

                if template_attribute.skills.is_none() {
                    continue;   
                }
                return Err(CharacterSheetError::SkillsMissingInAttribute(attribute.name.clone()));

            }

            let sheet_skills    = sheet_skills.unwrap();
            let template_skills = template_attribute.skills.as_ref().unwrap();

            for skill in sheet_skills {

                if !template_skills.iter().any(|ts| ts.name == skill.name) {
                    return Err(CharacterSheetError::SkillNotAllowed(skill.name.clone()));
                }

            }

        }

        Ok(())

    }

    fn valid_skill_allotment(&self, sheet: &CharacterSheet) -> Result<(), CharacterSheetError> {

        let mut s_total_points: i64 = 0;
        let t_allotments = self.allotments.skills.as_ref().unwrap();
        let max_points_per_allotment = t_allotments.max_points_per_allotment.unwrap_or(i64::MAX);

        for attribute in sheet.attributes.iter() {

            let sheet_skills = attribute.skills.as_ref();
            if sheet_skills.is_none() {
                continue;
            }

            let sheet_skills = sheet_skills.unwrap();
            s_total_points += sheet_skills
                .iter()
                .try_fold(0i64, |acc, skill| {

                    if skill.value < 0 {

                        return Err(CharacterSheetError::NegativeSkillPoints {
                            offending_skill: attribute.name.clone(),
                            points: skill.value,
                        });

                    }

                    if skill.value > max_points_per_allotment {

                        return Err(CharacterSheetError::TooManySkillPoints {
                            skill: skill.name.clone(),
                            allotted_points: skill.value,
                            max_points: max_points_per_allotment,
                        });

                    }

                    Ok(acc + skill.value)

                })?;

        }

        if s_total_points > t_allotments.given_points {
            return Err(CharacterSheetError::SkillPointsExceeded(s_total_points));
        }

        Ok(())

    }

}