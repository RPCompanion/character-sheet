
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character_sheet::CharacterSheet;
use crate::character_template::CharacterTemplate;


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

    pub fn roll(&self) -> CharacterRollResult {

        let mut rng   = thread_rng();
        let base_roll = rng.gen_range(1..=20);
        
        let modifier = match &self.roll_type {
            RollTarget::Attribute(attr) => Self::get_attribute_modifier(&attr),
            RollTarget::Skill(skill)     => Self::get_skill_modifier(&skill)
        };

        CharacterRollResult {
            target: self.roll_type.to_string(),
            value: base_roll + modifier,
            roll: base_roll,
            modifier: modifier
        }

    }

    fn get_attribute_modifier(attr: &str) -> i64 {

        todo!();

    }

    fn get_skill_modifier(skill: &str) -> i64 {

        todo!();
        
    }

}