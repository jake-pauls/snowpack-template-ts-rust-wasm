// Use 'extern' for importing external code, ie: JS functions
// Now add your own functions and abstractions! Happy Hacking!

use wasm_bindgen::prelude::*;

mod page;

/// The 'wasm_bindgen' attribute targets functions for the wasm binary
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// When tagged with (start), 'wasm_bindgen' executes the decorated function on page load
#[wasm_bindgen(start)]
pub fn start() {
    let _header = page::elements::random_header();
    let _description = page::elements::description();
    let _code = page::elements::code_block();
    let _congrats = page::elements::congrats();
}