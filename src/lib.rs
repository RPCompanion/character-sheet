#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod character_template;
pub mod character_sheet;

/*

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

*/