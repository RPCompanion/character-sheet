
use std::sync::LazyLock;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CharacterNameConfig {
    pub min_length: i32,
    pub max_length: i32
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CharacterDescriptionConfig {
    pub max_length: i32
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CharacterSheetConfig {
    pub name: CharacterNameConfig,
    pub description: CharacterDescriptionConfig
}

static RAW_CHARACTER_SHEET_CONFIG: &str = include_str!("../../CharacterSheet.toml");

pub static CHARACTER_SHEET_CONFIG: LazyLock<CharacterSheetConfig> = LazyLock::new(|| {
    toml::from_str(RAW_CHARACTER_SHEET_CONFIG).unwrap()
});