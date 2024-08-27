
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

    pub fn validate(&self) -> Result<(), CharacterSheetError> {

        self.validate_name()?;
        self.validate_description()?;
        Ok(())

    }

    pub fn validate_name(&self) -> Result<(), CharacterSheetError> {

        let name_conf = &config::get_character_sheet_config().name;

        if self.name.len() < name_conf.min_length as usize {
            return Err(CharacterSheetError::NameTooShort);
        }

        if self.name.len() > name_conf.max_length as usize {
            return Err(CharacterSheetError::NameTooLong);
        }

        Ok(())

    }

    fn validate_description(&self) -> Result<(), CharacterSheetError> {

        if let Some(description) = &self.description {

            if (description.len() as i32) > config::get_character_sheet_config().description.max_length {
                return Err(CharacterSheetError::DescriptionTooLong);
            }

        }

        Ok(())

    }

}

#[derive(Error, Debug, PartialEq)]
pub enum CharacterSheetError {
    
    #[error("Name too short")]
    NameTooShort,
    #[error("Name too long")]
    NameTooLong,

    #[error("Description too long")]
    DescriptionTooLong,
    #[error("Character template name mismatch")]
    NameMismatch,
    #[error("Character template version mismatch")]
    VersionMismatch,

    #[error("Character template does not allow perks")]
    PerksNotAllowed,
    #[error("Character template does not allow {0} as a perk")]
    PerkNotAllowed(String),
    #[error("Character template does not allow {0} perk points")]
    NotEnoughPerkPoints(i64),
    #[error("Character template does not allow more than {max_perks} perks, but {selected_perks} were selected")]
    TooManyPerks {
        selected_perks: i64,
        max_perks: i64
    },

    #[error("Character template does not allow {0} as an attribute")]
    AttributeNotAllowed(String),
    #[error("Character template does not allow this many points for a single attribute")]
    TooManyAttributePoints {
        attribute: String,
        allotted_points: i64,
        max_points: i64,
    },
    #[error("Character template does not allow negative attribute points for {offending_attribute} attribute")]
    NegativeAttributePoints {
        offending_attribute: String,
        points: i64
    },
    #[error("Character template does not allow {0} attribute points")]
    AttributePointsExceeded(i64),

    #[error("Character template does not allow {0} as a skill")]
    SkillNotAllowed(String),
    #[error("Character template does not have these skills at all. Is this an old sheet?")]
    SheetSkillsNotPresentInTemplateAttribute{
        attribute: String,
        skills: Vec<String>
    },
    #[error("Character template requires skills array for {0} attribute")]
    SkillsMissingInAttribute(String),
    #[error("Character template does not allow this many points for a single skill")]
    TooManySkillPoints {
        skill: String,
        allotted_points: i64,
        max_points: i64,
    },
    #[error("Character template does not allow negative skill points for {offending_skill} skill")]
    NegativeSkillPoints {
        offending_skill: String,
        points: i64
    },
    #[error("Character template does not allow {0} skill points")]
    SkillPointsExceeded(i64),

}