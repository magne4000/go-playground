use wasm_bindgen::prelude::*;
use web_sys::{console};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn log_all(some_iterable: &JsValue) {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let iterator = js_sys::try_iter(some_iterable).or_else(|_| Err("need to pass iterable JS values!"));

    for x in iterator.unwrap().unwrap() {
        console::log_1(&x.ok().unwrap());
    }
}