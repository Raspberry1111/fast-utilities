///! Shallow and deep cloning

use js_sys::{ArrayBuffer, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "v8")]
extern "C" {
    fn serialize(obj: &JsValue) -> ArrayBuffer;
    fn deserialize(obj: ArrayBuffer) -> JsValue;

}


/// Clones all top level properties on an object
#[wasm_bindgen(js_name = shallowClone)]
pub fn shallow_clone(x: JsValue) -> JsValue {
    if !x.is_object() {
        return x;
    }
    Object::assign(&Object::new(), &Object::from(x)).into()
}

/// Clones deeply. Requires v8 package to be present
#[wasm_bindgen(js_name = deepClone)]
pub fn deep_clone(x: JsValue) -> JsValue {
    if !x.is_object() {
        return x;
    }
    deserialize(serialize(&Object::from(x)))
}
