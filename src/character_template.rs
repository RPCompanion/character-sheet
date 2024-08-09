
use serde::{Deserialize, Serialize};

pub mod perk;
pub mod weapon_proficiency;
pub mod attributes;
pub mod common;

use attributes::Attribute;
use perk::Perk;
use weapon_proficiency::WeaponProficiency;

use crate::character_sheet::{self, CharacterSheet, CharacterSheetError};

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
pub struct PerkPoints {
    pub given_points: i64,
    pub max_perks: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Allotment {
    pub attributes: Points,
    pub skills: Option<Points>,
    pub perks: Option<PerkPoints>,
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

    pub fn get_base_character_sheet(&self) -> CharacterSheet {

        CharacterSheet {
            name: String::new(),
            template: character_sheet::Template {
                name: self.name.clone(),
                version: self.version.clone(),
            },   
            description: None,
            health: self.base_health,
            armor_class: self.base_armor_class,
            weapon_proficiencies: vec![],
            perks: self.perks.as_ref().map(|_| vec![]),
            attributes: self.attributes.iter().map(|a| {

                character_sheet::Attribute {

                    name: a.name.clone(),
                    value: 0,
                    skills: a.skills.as_ref().map(|s| {
                        s.iter().map(|s| {
                            character_sheet::Skill {
                                name: s.name.clone(),
                                value: 0,
                            }
                        }).collect()
                    })

                }  

            })
            .collect()

        }

    }

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

            if sheet_perks.len() > perk_points.max_perks.unwrap_or(i64::MAX) as usize {
                return Err(CharacterSheetError::TooManyPerks { selected_perks: sheet_perks.len() as i64, max_perks: perk_points.max_perks.unwrap() });
            }

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

            // If I've gotten here, then the sheet has skills for this attribute and the template might not.
            if template_attribute.skills.is_none() && sheet_skills.is_some_and(|s| !s.is_empty()) {
                
                return Err(CharacterSheetError::SheetSkillsNotPresentInTemplateAttribute{
                    attribute: attribute.name.clone(),
                    skills: sheet_skills.unwrap().iter().map(|s| s.name.clone()).collect()
                });

            }

            let sheet_skills    = sheet_skills.unwrap();
            let template_skills = dbg!(template_attribute.skills.as_ref().unwrap());

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


#[cfg(test)]
mod sheet_validation_tests {

    use crate::character_sheet::CharacterSheetError;
    use crate::character_sheet::config;
    use super::*;

    use json5;

    const STANDARD_TEMPLATE_STR: &str = include_str!("../standard.json5");

    fn get_unfailable_sheet(template: &CharacterTemplate) -> CharacterSheet {

        let mut sheet = template.get_base_character_sheet();

        let config = config::get_character_sheet_config();
        sheet.name = (0..config.name.min_length).map(|_| 'a').collect();
        sheet

    }

    fn get_template_and_sheet() -> (CharacterTemplate, CharacterSheet) {

        let template: CharacterTemplate = json5::from_str(STANDARD_TEMPLATE_STR).unwrap();
        let sheet = get_unfailable_sheet(&template);

        (template, sheet)

    }

    #[test]
    fn deserialization() {
        assert!(json5::from_str::<CharacterTemplate>(STANDARD_TEMPLATE_STR).is_ok());
    }

    #[test]
    fn unfailable_sheet_test() {

        let (template, sheet) = get_template_and_sheet();
        let response = template.validate_sheet(&sheet);
        assert!(response.is_ok());

    }

    #[test]
    fn short_name_test() {

        let (template, mut sheet) = get_template_and_sheet();
        sheet.name   = "".to_string();

        let response = template.validate_sheet(&sheet);
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NameTooShort);

    }

    #[test]
    fn long_name_test() {

        let (template, mut sheet) = get_template_and_sheet();

        let config = config::get_character_sheet_config();
        sheet.name = (0..config.name.max_length+1).map(|_| 'a').collect();

        let response = template.validate_sheet(&sheet);
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NameTooLong);

    }

    #[test]
    fn template_name_test() {

        let (template, mut sheet) = get_template_and_sheet();
        sheet.template.name = "Not the same name".to_string();

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NameMismatch);

    }

    #[test]
    fn template_version_test() {

        let (template, mut sheet) = get_template_and_sheet();
        sheet.template.version = [0, 0, 0];

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::VersionMismatch);

    }

    #[test]
    fn perks_not_allowed_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.perks = Some(vec!["Perk".to_string()]);

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::PerkNotAllowed("Perk".to_string()));

    }

    #[test]
    fn perks_not_enough_points_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.perks = Some(vec!["Force Sensitive".to_string(), "Small Frame".to_string(), "Charismatic".to_string()]);

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NotEnoughPerkPoints(5));

    }

    #[test]
    fn attribute_not_allowed_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.attributes.push(character_sheet::Attribute {
            name: "Not an attribute".to_string(),
            value: 0,
            skills: None,
        });

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::AttributeNotAllowed("Not an attribute".to_string()));

    }

    #[test]
    fn attribute_points_exceeded_test() {

        let (template, mut sheet) = get_template_and_sheet();

        let max_points_per_allotment = &template.allotments.attributes.max_points_per_allotment;
        let given_points = template.allotments.attributes.given_points;

        sheet.attributes
            .iter_mut()
            .for_each(|a| a.value = max_points_per_allotment.unwrap_or(given_points));

        let points_assigned = sheet.attributes
            .iter()
            .map(|a| a.value)
            .sum::<i64>();

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::AttributePointsExceeded(points_assigned));

    }

    #[test]
    fn attribute_too_many_points_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.attributes[0].value = template.allotments.attributes.given_points + 1;

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::TooManyAttributePoints {
            attribute: sheet.attributes[0].name.clone(),
            allotted_points: template.allotments.attributes.given_points + 1,
            max_points: template.allotments.attributes.max_points_per_allotment.unwrap_or(i64::MAX),
        });

    }

    #[test]
    fn negative_attribute_points_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.attributes[0].value = -1;

        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NegativeAttributePoints {
            offending_attribute: sheet.attributes[0].name.clone(),
            points: -1,
        });

    }

    #[test]
    fn skills_not_allowed_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.attributes[0].skills = Some(vec![
            character_sheet::Skill {
                name: "Not a skill".to_string(),
                value: 0,
            }
        ]);

        let response = template.validate_sheet(&sheet);
        let expected_error = CharacterSheetError::SheetSkillsNotPresentInTemplateAttribute { 
            attribute: sheet.attributes[0].name.clone(),
            skills: vec!["Not a skill".to_string()]
        };

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);

    }

    #[test]
    fn skill_points_exceeded_test() {

        let (template, mut sheet) = get_template_and_sheet();

        let max_points_per_allotment = template.allotments.skills.as_ref().unwrap().max_points_per_allotment;
        let given_points = template.allotments.skills.as_ref().unwrap().given_points;

        let attr = sheet.attributes
            .iter_mut()
            .find(|f| {
                f.skills.as_ref().is_some_and(|f| !f.is_empty())
            });

        if attr.is_none() {
            panic!("No attribute with skills found, unable to test skill points exceeded");
        }

        let attr = attr.unwrap();
        attr.skills.as_mut().unwrap()[0].value = max_points_per_allotment.unwrap_or(given_points) + 1;

        let expected_response = CharacterSheetError::TooManySkillPoints { 
            skill: attr.skills.as_ref().unwrap()[0].name.clone(), 
            allotted_points: max_points_per_allotment.unwrap_or(given_points) + 1, 
            max_points: max_points_per_allotment.unwrap_or(i64::MAX)
        };
        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_response);

    }

    #[test]
    fn negative_skill_points_test() {

        let (template, mut sheet) = get_template_and_sheet();

        let attr = sheet.attributes
            .iter_mut()
            .find(|f| {
                f.skills.as_ref().is_some_and(|f| !f.is_empty())
            });

        if attr.is_none() {
            panic!("No attribute with skills found, unable to test negative skill points");
        }

        let attr = attr.unwrap();
        attr.skills.as_mut().unwrap()[0].value = -1;

        let expected_response = CharacterSheetError::NegativeSkillPoints { 
            offending_skill: attr.name.clone(), 
            points: -1 
        };

        let response = template.validate_sheet(&sheet);
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_response);

    }

    #[test]
    fn skills_too_many_points_test() {

        let (template, mut sheet) = get_template_and_sheet();

        let max_points_per_allotment = template.allotments.skills.as_ref().unwrap().max_points_per_allotment;
        let given_points = template.allotments.skills.as_ref().unwrap().given_points;
        let skill_value = max_points_per_allotment.unwrap_or(given_points);

        let mut total_points_allocated: i64 = 0;
        sheet.attributes
            .iter_mut()
            .filter(|attr| {
                attr.skills.as_ref().is_some_and(|f| !f.is_empty())
            })
            .for_each(|s| {
                s.skills.as_mut().unwrap()[0].value = skill_value;
                total_points_allocated += skill_value;
            });

        let expected_response = CharacterSheetError::SkillPointsExceeded(total_points_allocated);
        let response = template.validate_sheet(&sheet);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_response);

    }

}