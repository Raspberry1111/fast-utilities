///! Clamps a value to be in between a range

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn clamp(number: f64, minimum: f64, maximum: f64) -> f64 {
    if minimum > maximum {
        return number;
    };
    maximum.min(number.max(minimum))
}
