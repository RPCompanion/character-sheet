
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CharacterNameConfig {
    pub min_length: i32,
    pub max_length: i32
}

#[derive(Serialize, Deserialize)]
pub struct CharacterDescriptionConfig {
    pub max_length: i32
}

#[derive(Serialize, Deserialize)]
pub struct CharacterSheetConfig {
    pub name: CharacterNameConfig,
    pub description: CharacterDescriptionConfig
}

const RAW_CHARACTER_SHEET_CONFIG: &str = include_str!("../../CharacterSheet.toml");

static CHARACTER_SHEET_CONFIG: OnceLock<CharacterSheetConfig> = OnceLock::new();

pub fn get_character_sheet_config() -> &'static CharacterSheetConfig {

    CHARACTER_SHEET_CONFIG.get_or_init(|| {
        toml::from_str(RAW_CHARACTER_SHEET_CONFIG).unwrap()
    })

}