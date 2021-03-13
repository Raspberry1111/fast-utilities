///! Several Functions built into rust exported
use wasm_bindgen::prelude::*;

/// Clamp a number to be between minimum and maximum
#[wasm_bindgen]
pub fn clamp(number: f64, minimum: f64, maximum: f64) -> f64 {
    number.clamp(minimum, maximum)
}

/// Round a number up
#[wasm_bindgen]
pub fn ceil(number: f64) -> f64 {
    number.ceil()
}

/// Round a number down
#[wasm_bindgen]
pub fn floor(number: f64) -> f64 {
    number.floor()
}

/// Returns the absolute value of a number
#[wasm_bindgen]
pub fn abs(number: f64) -> f64 {
    number.abs()
}

/// Rounds a number
#[wasm_bindgen]
pub fn round(number: f64) -> f64 {
    number.round()
}
