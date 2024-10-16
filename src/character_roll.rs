
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::character_sheet::CharacterSheet;
use crate::character_template::common::{AttributeModifier, SkillModifier};
use crate::character_template::CharacterTemplate;


#[derive(Error, Debug, PartialEq)]
pub enum RollError {
    #[error("Invalid attribute {0}")]
    InvalidAttribute(String),
    #[error("Invalid skill {0}")]
    InvalidSkill(String)
}

#[derive(Serialize, Deserialize)]
pub enum RollTarget {
    Attribute(String),
    Skill(String)
}

impl RollTarget {

    pub fn to_string(&self) -> String {

        match self {
            RollTarget::Attribute(attr) => attr.clone(),
            RollTarget::Skill(skill)     => skill.clone()
        }

    }

    pub fn as_str(&self) -> &str {

        match self {
            RollTarget::Attribute(attr) => attr.as_str(),
            RollTarget::Skill(skill)     => skill.as_str()
        }

    }

}

pub struct CharacterRoll<'a> {
    template: &'a CharacterTemplate,
    sheet: &'a CharacterSheet,
    roll_type: RollTarget
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterRollResult {
    pub target: String,
    pub value: i64,
    pub roll: i64,
    pub modifier: i64
}

impl<'a> CharacterRoll<'a> {

    pub fn new(template: &'a CharacterTemplate, sheet: &'a CharacterSheet, roll_type: RollTarget) -> Self {
        Self {
            template,
            sheet,
            roll_type
        }
    }

    pub fn roll(&self) -> Result<CharacterRollResult, RollError> {

        let mut rng   = thread_rng();
        let base_roll = rng.gen_range(1..=20);
        
        let modifier = match &self.roll_type {
            RollTarget::Attribute(_) => self.get_attribute_modifier(),
            RollTarget::Skill(_)     => self.get_skill_modifier()
        }?;

        Ok(CharacterRollResult {
            target: self.roll_type.to_string(),
            value: base_roll + modifier,
            roll: base_roll,
            modifier: modifier
        })

    }

    fn get_attribute_modifier(&self) -> Result<i64, RollError> {

        if let Some(attr) = self.sheet.attributes.iter().find(|a| a.name == self.roll_type.as_str()) {
            return Ok(attr.value + self.perk_modifier());
        }

        Err(RollError::InvalidAttribute(self.roll_type.to_string()))

    }

    fn get_skill_modifier(&self) -> Result<i64, RollError> {

        let perk_modifier = self.perk_modifier();

        let attr = self.sheet.attributes
            .iter()
            .find(|a| {

                if a.skills.is_none() {
                    return false;
                }

                let skill_name = self.roll_type.as_str();

                a.skills.as_ref().unwrap()
                    .iter()
                    .find(|s| s.name == skill_name)
                    .is_some()

            });

        if attr.is_none() {
            return Err(RollError::InvalidSkill(self.roll_type.to_string()));
        }

        let attr = attr.unwrap();
        let skill = attr.skills
            .as_ref()
            .unwrap()
            .iter()
            .find(|s| s.name == self.roll_type.as_str())
            .unwrap();

        Ok(skill.value + attr.value + perk_modifier)

    }

    fn perk_modifier(&self) -> i64 {

        if self.template.perks.is_none() || self.sheet.perks.is_none() {
            return 0;
        }

        let t_perks = self.template.perks.as_ref().unwrap();
        let s_perks = self.sheet.perks.as_ref().unwrap();

        let modifier: i64 = match &self.roll_type {

            RollTarget::Attribute(attr) => {

                let attr_modifiers: Vec<&AttributeModifier> = t_perks
                    .iter()
                    .filter(|p| s_perks.contains(&p.name)) // Filter out perks that the character doesn't have
                    .filter(|f| f.attributes.is_some())    // Filter out perks that don't have attribute modifiers
                    .map(|p| p.attributes.as_ref().unwrap()) // Get the attribute modifiers
                    .flatten()
                    .filter(|t_attr| t_attr.name == *attr) // Filter out attribute modifiers that don't match the target attribute
                    .collect();

                attr_modifiers.iter().map(|m| m.modifier).sum()

            },
            RollTarget::Skill(skill) => {

                let skill_modifiers: Vec<&SkillModifier> = t_perks
                    .iter()
                    .filter(|p| s_perks.contains(&p.name)) // Filter out perks that the character doesn't have
                    .filter(|f| f.skills.is_some())    // Filter out perks that don't have skill modifiers
                    .map(|p| p.skills.as_ref().unwrap()) // Get the skill modifiers
                    .flatten()
                    .filter(|t_skill| t_skill.name == *skill) // Filter out skill modifiers that don't match the target skill
                    .collect();

                skill_modifiers.iter().map(|m| m.modifier).sum()

            }

        };

        modifier

    }

}

#[cfg(test)]
mod character_roll_tests {

    use std::sync::LazyLock;
    use super::*;

    const STANDARD_TEMPLATE_STR: &str = include_str!("../standard.json");
    static STANDARD_TEMPLATE: LazyLock<CharacterTemplate> = LazyLock::new(|| {
        CharacterTemplate::from_json_str(STANDARD_TEMPLATE_STR.to_string()).unwrap()
    });

    const CHARACTER_SHEET_SAMPLE_STR: &str = include_str!("../character_sheet_sample.json");
    static CHARACTER_SHEET_SAMPLE: LazyLock<CharacterSheet> = LazyLock::new(|| {
        serde_json::from_str(CHARACTER_SHEET_SAMPLE_STR).unwrap()
    });

    #[test]
    fn test_roll_attribute() {

        let character = CharacterRoll {
            template: &STANDARD_TEMPLATE,
            sheet: &CHARACTER_SHEET_SAMPLE,
            roll_type: RollTarget::Attribute("Strength".to_string())
        };

        let roll = character.roll();
        assert!(roll.is_ok());

    }

    #[test]
    fn test_roll_fake_attribute() {

        let character = CharacterRoll {
            template: &STANDARD_TEMPLATE,
            sheet: &CHARACTER_SHEET_SAMPLE,
            roll_type: RollTarget::Attribute("Fake".to_string())
        };

        let roll = character.roll();
        assert!(roll.is_err());
        assert!(roll.unwrap_err() == RollError::InvalidAttribute("Fake".to_string()));

    }

    #[test]
    fn test_roll_skill() {

        let character = CharacterRoll {
            template: &STANDARD_TEMPLATE,
            sheet: &CHARACTER_SHEET_SAMPLE,
            roll_type: RollTarget::Skill("Acrobatics".to_string())
        };

        let roll = dbg!(character.roll());
        assert!(roll.is_ok());

    }

}