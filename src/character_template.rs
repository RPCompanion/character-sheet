
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

pub mod perk;
pub mod weapon_proficiency;
pub mod attributes;
pub mod common;

use attributes::Attribute;
use perk::Perk;
use weapon_proficiency::WeaponProficiency;

use crate::character_sheet::{self, CharacterSheet};

/**
 * 
 * How many points a character has for a particular attribute/skill/perk
 * and how many points they can allocate to a single attribute/skill/perk
 * 
*/
#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Points {
    pub given_points: i64,
    pub max_points_per_allotment: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct PerkPoints {
    pub given_points: i64,
    pub max_perks: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Allotment {
    pub attributes: Points,
    pub skills: Option<Points>,
    pub perks: Option<PerkPoints>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

}