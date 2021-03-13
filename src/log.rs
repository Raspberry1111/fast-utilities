///! Provides methods to log to console
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: JsValue);

}

pub fn log_value<T: Into<JsValue>>(x: T) {
    log(x.into())
}
