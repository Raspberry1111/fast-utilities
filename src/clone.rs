///! Shallow and deep cloning
use js_sys::{
    Array, ArrayBuffer, Object,
    Reflect::{get, set},
};
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
* Clones all properties on an object. Does not support cyclic objects
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

// Was going for something like https://stackoverflow.com/a/34624648
/// Clones all properties on an object. Does not support cyclic objects
#[wasm_bindgen(js_name = deepClone, skip_typescript)]
pub fn deep_clone(value: JsValue) -> JsValue {
    if !value.is_object() || value.is_null() {
        return value;
    };
    let value_object = Object::from(value);
    let cloned_object = Object::new() as Object;
    let keys = Object::keys(&value_object) as Array;
    for key in keys.to_vec() {
		// We are getting the descriptor because I think to keep attributes like read-only you need to get it
        let descriptor: Object = Object::get_own_property_descriptor(&value_object, &key).into();
        let value = get(&descriptor, &JsValue::from_str("value")).unwrap();

		// We reassign to the descriptors value to make sure we clone all nested objects
        set(&descriptor, &JsValue::from_str("value"), &deep_clone(value)).unwrap();
        Object::define_property(&cloned_object, &key, &descriptor);
    }
    cloned_object.into()
}
