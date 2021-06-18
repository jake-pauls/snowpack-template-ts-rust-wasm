use web_sys::{Window, Document, HtmlElement};

pub fn window() -> Window {
    web_sys::window()
        .expect("Window context not rendered yet")
}

pub fn document() -> Document {
    window()
        .document()
        .expect("Document context should be present on window")
}

pub fn body() -> HtmlElement {
    document()
        .body()
        .expect("Body context should be present within document")
}