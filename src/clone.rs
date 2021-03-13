///! Shallow and deep cloning
use js_sys::{ArrayBuffer, Object};
use wasm_bindgen::prelude::*;

// We need this to use generics for better types
#[wasm_bindgen(typescript_custom_section)]
const CLONE_FUNCTIONS: &'static str = r#"
type CloneFn = <T>(object: T) => T; 

/**
* Clones all top level properties on an object
*/
export const shallowClone: CloneFn
/**
* Clones all properties on an object
*/
export const deepClone: CloneFn
"#;

#[wasm_bindgen(module = "v8")]
extern "C" {
    fn serialize(object: &JsValue) -> ArrayBuffer;
    fn deserialize(buffer: ArrayBuffer) -> JsValue;

}

/// Clones all top level properties on an object
#[wasm_bindgen(js_name = shallowClone, skip_typescript)]
pub fn shallow_clone(value: JsValue) -> JsValue {
    if !value.is_object() {
        return value;
    };
    let object = Object::from(value);
    Object::assign(&Object::new(), &object).into()
}

/// Clones deeply. Requires v8 package to be present
#[wasm_bindgen(js_name = deepClone, skip_typescript)]
pub fn deep_clone(value: JsValue) -> JsValue {
    if !value.is_object() {
        return value;
    };
    let object = Object::from(value);
    deserialize(serialize(&object))
}
