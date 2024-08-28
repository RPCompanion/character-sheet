use character_sheet::config::{CHARACTER_SHEET_CONFIG, CharacterSheetConfig};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use js_sys::JsString;

pub mod character_template;
pub mod character_sheet;
pub mod character_sheet_validator;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_sheet_config() -> CharacterSheetConfig {
    *CHARACTER_SHEET_CONFIG
}

#[cfg(target_arch = "wasm32")]
type InteralString = JsString;
#[cfg(not(target_arch = "wasm32"))]
type InteralString = String;

#[cfg(target_arch = "wasm32")]
type InternalVersion = Vec<u8>;
#[cfg(not(target_arch = "wasm32"))]
type InternalVersion = [u8; 3];