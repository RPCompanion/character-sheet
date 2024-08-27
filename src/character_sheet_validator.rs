
use thiserror::Error;

use crate::character_sheet::config::CHARACTER_SHEET_CONFIG;
use crate::character_template::CharacterTemplate;
use crate::character_sheet::CharacterSheet;

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





pub struct CharacterSheetValidator<'a> {
    template: &'a CharacterTemplate,
    sheet: &'a CharacterSheet
}

impl<'a> CharacterSheetValidator<'a> {

    pub fn new(template: &'a CharacterTemplate, sheet: &'a CharacterSheet) -> Self {

        Self {
            template,
            sheet
        }

    }

    /// Checks the character sheet against the template and config.
    pub fn check(&self) -> Result<(), CharacterSheetError> {

        self.validate_character_name()?;
        self.validate_character_desc()?;
        self.check_template_name()?;
        self.check_version()?;
        self.check_perks()?;
        self.check_perk_allotment()?;
        self.check_attributes()?;
        self.check_attribute_allotment()?;
        self.check_skills()?;
        self.check_skill_allotment()?;
        Ok(())

    }

    pub fn validate_character_name(&self) -> Result<(), CharacterSheetError> {

        let name_conf = CHARACTER_SHEET_CONFIG.name;

        if self.sheet.name.len() < name_conf.min_length as usize {
            return Err(CharacterSheetError::NameTooShort);
        }

        if self.sheet.name.len() > name_conf.max_length as usize {
            return Err(CharacterSheetError::NameTooLong);
        }

        Ok(())

    }

    pub fn validate_character_desc(&self) -> Result<(), CharacterSheetError> {
    
        if let Some(description) = &self.sheet.description {

            if (description.len() as i32) > CHARACTER_SHEET_CONFIG.description.max_length {
                return Err(CharacterSheetError::DescriptionTooLong);
            }

        }

        Ok(())

    }

    fn check_template_name(&self) -> Result<(), CharacterSheetError> {

        if self.sheet.template.name == self.template.name {
            return Ok(());
        }
        
        Err(CharacterSheetError::NameMismatch)

    }

    fn check_version(&self) -> Result<(), CharacterSheetError> {

        if self.sheet.template.version.len() != self.template.version.len() {
            return Err(CharacterSheetError::VersionMismatch);
        }

        if self.template.version.iter().eq(self.sheet.template.version.iter()) {
            return Ok(());
        }

        Err(CharacterSheetError::VersionMismatch)

    }

    fn check_perks(&self) -> Result<(), CharacterSheetError> {

        if self.sheet.perks.is_none() {
            return Ok(());
        }

        if self.template.perks.is_none() && self.sheet.perks.is_none() {

            return Ok(());

        } else if self.template.perks.is_none() && self.sheet.perks.is_some() {

            if self.sheet.perks.as_ref().unwrap().is_empty() {
                return Ok(());
            } else {
                return Err(CharacterSheetError::PerksNotAllowed);
            }

        } else if self.template.perks.is_some() && self.sheet.perks.is_some() {

            let template_perks = self.template.perks.as_ref().unwrap();
            let sheet_perks    = self.sheet.perks.as_ref().unwrap();

            for perk in sheet_perks {

                if !template_perks.iter().any(|tp| tp.name == *perk) {
                    return Err(CharacterSheetError::PerkNotAllowed(perk.clone()));
                }

            }


        }
        
        Ok(())

    }

    fn check_perk_allotment(&self) -> Result<(), CharacterSheetError> {

        if let Some(perk_points) = &self.template.allotments.perks {

            let template_perks = self.template.perks.as_ref().unwrap();
            let sheet_perks    = self.sheet.perks.as_ref().unwrap();

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

    fn check_attributes(&self) -> Result<(), CharacterSheetError> {

        for attribute in self.sheet.attributes.iter() {

            if !self.template.attributes.iter().any(|ta| ta.name == attribute.name) {
                return Err(CharacterSheetError::AttributeNotAllowed(attribute.name.clone()));
            }

        }

        Ok(())

    }

    fn check_attribute_allotment(&self) -> Result<(), CharacterSheetError> {

        let t_attr_points = &self.template.allotments.attributes;
        let max_points_per_allotment =  t_attr_points.max_points_per_allotment.unwrap_or(i64::MAX);

        let s_total_points = self.sheet.attributes
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

    fn check_skills(&self) -> Result<(), CharacterSheetError> {

        for attribute in self.sheet.attributes.iter() {

            let template_attribute = self.template.attributes
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
            let template_skills = template_attribute.skills.as_ref().unwrap();

            for skill in sheet_skills {

                if !template_skills.iter().any(|ts| ts.name == skill.name) {
                    return Err(CharacterSheetError::SkillNotAllowed(skill.name.clone()));
                }

            }

        }

        Ok(())

    }

    fn check_skill_allotment(&self) -> Result<(), CharacterSheetError> {

        let mut s_total_points: i64 = 0;
        let t_allotments = self.template.allotments.skills.as_ref().unwrap();
        let max_points_per_allotment = t_allotments.max_points_per_allotment.unwrap_or(i64::MAX);

        for attribute in self.sheet.attributes.iter() {

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

    use crate::character_sheet;
    use crate::character_sheet::config::CHARACTER_SHEET_CONFIG;
    use super::*;

    use json5;

    const STANDARD_TEMPLATE_STR: &str = include_str!("../standard.json5");

    fn get_unfailable_sheet(template: &CharacterTemplate) -> CharacterSheet {

        let mut sheet = template.get_base_character_sheet();

        let config = *CHARACTER_SHEET_CONFIG;
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

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_ok());

    }

    #[test]
    fn short_name_test() {

        let (template, mut sheet) = get_template_and_sheet();
        sheet.name   = "".to_string();

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NameTooShort);

    }

    #[test]
    fn long_name_test() {

        let (template, mut sheet) = get_template_and_sheet();

        let config = *CHARACTER_SHEET_CONFIG;
        sheet.name = (0..config.name.max_length+1).map(|_| 'a').collect();

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NameTooLong);

    }

    #[test]
    fn template_name_test() {

        let (template, mut sheet) = get_template_and_sheet();
        sheet.template.name = "Not the same name".to_string();

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::NameMismatch);

    }

    #[test]
    fn template_version_test() {

        let (template, mut sheet) = get_template_and_sheet();
        sheet.template.version = [0, 0, 0];

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::VersionMismatch);

    }

    #[test]
    fn perks_not_allowed_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.perks = Some(vec!["Perk".to_string()]);

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::PerkNotAllowed("Perk".to_string()));

    }

    #[test]
    fn perks_not_enough_points_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.perks = Some(vec!["Force Sensitive".to_string(), "Small Frame".to_string(), "Charismatic".to_string()]);

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

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

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

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

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), CharacterSheetError::AttributePointsExceeded(points_assigned));

    }

    #[test]
    fn attribute_too_many_points_test() {

        let (template, mut sheet) = get_template_and_sheet();

        sheet.attributes[0].value = template.allotments.attributes.given_points + 1;

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

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

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

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

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

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

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

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

        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

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
        
        let validator = CharacterSheetValidator::new(&template, &sheet);
        let response = validator.check();

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_response);

    }

}