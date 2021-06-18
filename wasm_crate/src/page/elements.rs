use super::dom::{document, body};
use wasm_bindgen::prelude::*;
use rand::prelude::SliceRandom;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub const DESCRIPTION: &str = "Snowpack uses HMR to manually build and serve your WebAssembly crates as you work.\nHowever, if you'd like to, here's how you can manually build and test your crate:"; 

fn center_element(element: &HtmlElement) {
    let _ = element.style().set_property("text-align", "center");
}

pub fn random_header() -> Result<(), JsValue> {
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

pub fn description() -> Result<(), JsValue> {
    let description = document()
        .create_element("p")?
        .dyn_into::<HtmlElement>()?;

    description.set_inner_text(DESCRIPTION);
    center_element(&description);

    body().append_child(&description)?;

    Ok(())
}

pub fn code_block() -> Result<(), JsValue> {
    let code = document()
        .create_element("p")?
        .dyn_into::<HtmlElement>()?;

    code.set_inner_html("<code>yarn wasm:build   &&   yarn wasm:test</code>");
    center_element(&code);

    body().append_child(&code)?;

    Ok(())
}

pub fn congrats() -> Result<(), JsValue> {
    let congrats = document()
        .create_element("p")?
        .dyn_into::<HtmlElement>()?;

    congrats.set_inner_text("Happy Hacking!");
    center_element(&congrats);

    body().append_child(&congrats)?;

    Ok(())
}