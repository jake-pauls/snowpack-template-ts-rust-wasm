// Use 'extern' for importing external code, ie: JS functions
// The 'wasm_bindgen' attribute targets functions for the wasm binary
// When tagged with (start), 'wasm_bindgen' runs on page load
//
// Now add your own functions and abstractions! Happy Hacking!

use wasm_bindgen::prelude::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlElement};

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(start)]
pub fn start() {
    let _header = random_header();
    let _description = description();
    let _code = code_block();
    let _congrats = congrats();
}

fn window() -> Window {
    web_sys::window()
        .expect("Window context not rendered yet")
}

fn document() -> Document {
    window()
        .document()
        .expect("Document context should be present on window")
}

fn body() -> HtmlElement {
    document()
        .body()
        .expect("Body context should be present within document")
}

fn center_element(element: &HtmlElement) {
    let _ = element.style().set_property("text-align", "center");
}

fn random_header() -> Result<(), JsValue> {
    let headings: [&str; 4] = [
        "Welcome to Rust Snowpack!",
        "Rustaceans might not like the snow...",
        "Do crabs like the snow?",
        "Thanks for supporting the template!"
    ];

    // Spin a random header
    let mut rng = rand::thread_rng();
    let text = headings.choose(&mut rng).unwrap().to_string();

    let heading = document()
        .create_element("h1")?
        .dyn_into::<HtmlElement>()?;

    heading.set_inner_html(&text);
    center_element(&heading);

    let _res = body().append_child(&heading);

    Ok(())
}

fn description() -> Result<(), JsValue> {
    let description = document()
        .create_element("p")?
        .dyn_into::<HtmlElement>()?;

    description.set_inner_text("Snowpack uses HMR to manually build and serve WebAssembly.\nHowever, if you'd like to, here's how you manually build and test your crate:");
    center_element(&description);

    body().append_child(&description)?;

    Ok(())
}

fn code_block() -> Result<(), JsValue> {
    let code = document()
        .create_element("p")?
        .dyn_into::<HtmlElement>()?;

    code.set_inner_html("<code>yarn wasm:build   &&   yarn wasm:test</code>");
    center_element(&code);

    body().append_child(&code)?;

    Ok(())
}

fn congrats() -> Result<(), JsValue> {
    let congrats = document()
        .create_element("p")?
        .dyn_into::<HtmlElement>()?;

    congrats.set_inner_text("Happy Hacking!");
    center_element(&congrats);

    body().append_child(&congrats)?;

    Ok(())
}
