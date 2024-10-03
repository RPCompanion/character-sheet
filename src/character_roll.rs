
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::character_sheet::CharacterSheet;
use crate::character_template::common::{AttributeModifier, SkillModifier};
use crate::character_template::CharacterTemplate;


#[derive(Error, Debug)]
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

#[derive(Serialize, Deserialize)]
pub struct CharacterRollResult {
    pub target: String,
    pub value: i64,
    pub roll: i64,
    pub modifier: i64
}

impl<'a> CharacterRoll<'a> {

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