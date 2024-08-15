use character_sheet::config::{get_character_sheet_config, CharacterSheetConfig};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod character_template;
pub mod character_sheet;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_sheet_config() -> CharacterSheetConfig {
    *get_character_sheet_config()
}