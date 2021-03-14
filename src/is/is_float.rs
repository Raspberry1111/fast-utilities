///! Checks whether the given input is a floating point number
use wasm_bindgen::prelude::*;

/// Checks whether value is a floating point number. Numbers ending in .0 are not considered floats.
/// NOTE: Any number above 2,147,483,647 (About 2.15 billion) returns true because wasm doesn't support 64 bit integers
#[wasm_bindgen(js_name = isFloat)]
pub fn is_float(value: JsValue) -> bool {
    // If its not a number, return false. if we didn't do this non numbers would return true
    if let Some(number) = value.as_f64() {
		// This checks if the number without the decimal part is NOT itself with the decimals
        ((number as i32) as f64 - number).abs() >= 0.01
    } else {
        false
    }
}
