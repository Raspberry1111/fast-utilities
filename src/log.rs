
///! Provides methods to log to console
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = console)]
extern "C" {
    fn log(s: &str);
    fn log_js_value(s: JsValue);

}