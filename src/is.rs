#![allow(clippy::float_cmp)] // We can allow this because we don't do anything aside from remove the decimals when compare
///! Checks whether the given input is a  somthing else
use wasm_bindgen::prelude::*;

use crate::log::log_value;

/// Checks whether value is a floating point number. Numbers ending in .0 are not considered floats.
/// NOTE: Any number above 2,147,483,647 (About 2.15 billion) returns true because wasm doesn't support 64 bit signed integers.
#[wasm_bindgen(js_name = isFloat)]
pub fn is_float(value: JsValue) -> bool {
    // If its not a number, return false. if we didn't do this non numbers would return true
    if let Some(number) = value.as_f64() {
        // This checks if the number without the decimals is the not same, if so its a float
        (number as i32) as f64 != number
    } else {
        false
    }
}

/// Checks whether value is an integer. Numbers ending in .0 are considered integers.
/// NOTE: Any number above 2,147,483,647 (About 2.15 billion) returns false because wasm doesn't support 64 bit signed integers
#[wasm_bindgen(js_name = isInteger)]
pub fn is_integer(value: JsValue) -> bool {
    log_value(&value);
    if value.as_f64().is_none() {
        return false;
    };
    !is_float(value)
}
