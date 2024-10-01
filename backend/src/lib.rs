use crate::ieee754_ops::{binary_to_decimal, FLOAT32_LAYOUT};
use ieee754_ops::decimal_to_binary;
use wasm_bindgen::prelude::*;

mod bitfield;
mod ieee754_ops;

// This will expose the function to JavaScript
#[wasm_bindgen]
pub fn float32_to_binary(float: &str) -> String {
    decimal_to_binary(float, FLOAT32_LAYOUT)
}

// An example of converting binary to a 32-bit float
#[wasm_bindgen]
pub fn binary_to_float32(binary: &str) -> String {
    binary_to_decimal(binary, FLOAT32_LAYOUT, 20)
}
