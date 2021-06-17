/**
 * Use 'extern' for importing external code, ie: JS functions
 * The 'wasm_bindgen' attribute targets functions for the wasm binary
 * When tagged with (start), 'wasm_bindgen' runs on page load
 *
 * Now add your own functions! Happy Hacking!
 */
use wasm_bindgen::prelude::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn window() -> web_sys::Window {
    web_sys::window().expect("Window context not rendered yet")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Document context should be present on window")
}

fn body() -> web_sys::HtmlElement {
    document()
        .body()
        .expect("Body context should be present within document")
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let headings: [&str; 3] = ["Welcome to Rust Snowpack!", "Rustaceans might not like the snow...", "Do crabs like the snow?"];

    let mut rng = rand::thread_rng();
    let text = headings.choose(&mut rng).unwrap().to_string();

    // Create, style, and append a DOM element
    let heading = document()
        .create_element("h1")?
        .dyn_into::<web_sys::HtmlElement>()?;

    heading.set_inner_html(&text);

    let _ = heading
        .style()
        .set_property("text-align", "center");

    body().append_child(&heading)?;

    Ok(())
}
