#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod character_template;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}