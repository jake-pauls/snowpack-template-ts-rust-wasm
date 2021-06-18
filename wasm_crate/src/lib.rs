// Use 'extern' for importing external code, ie: JS functions
// The 'wasm_bindgen' attribute targets functions for the wasm binary
// When tagged with (start), 'wasm_bindgen' runs on page load
//
// Now add your own functions and abstractions! Happy Hacking!

use wasm_bindgen::prelude::*;

mod page;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(start)]
pub fn start() {
    let _header = page::elements::random_header();
    let _description = page::elements::description();
    let _code = page::elements::code_block();
    let _congrats = page::elements::congrats();
}