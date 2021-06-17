use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
}

#[wasm_bindgen_test]
async fn promise_test() {
    // Create a client JS Promise
    let promise = js_sys::Promise::resolve(&JsValue::from(42));

    // Convert that promise into a Rust Future
    let future = JsFuture::from(promise).await.unwrap();

    assert_eq!(future, 42);
}
