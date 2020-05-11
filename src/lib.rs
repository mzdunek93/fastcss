mod utils;
mod js_interop;
use js_interop::{Date, log};
mod parse;
use parse::root;
mod structs;
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let start = Date::new().get_time();
    let (_, root) = root(input).unwrap();
    let end = Date::new().get_time();
    log(&format!("Parsing time: {}ms", end - start));
    JsValue::from_serde(&root).unwrap()
}
