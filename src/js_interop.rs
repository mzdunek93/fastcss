use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    pub type Date;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Date;

    #[wasm_bindgen(method, final, js_name = "getTime")]
    pub fn get_time(this: &Date) -> u32;
}
