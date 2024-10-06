use crate::ieee754_ops::{binary_to_decimal, binary_to_decimal_ext, BinaryInfo, FLOAT32_LAYOUT};
use ieee754_ops::decimal_to_binary;
use wasm_bindgen::prelude::*;

mod bitfield;
mod ieee754_ops;

#[wasm_bindgen]
pub fn float32_to_binary(float: &str) -> String {
    decimal_to_binary(float, &FLOAT32_LAYOUT)
}

#[wasm_bindgen]
pub fn binary_to_float32(binary: &str) -> String {
    binary_to_decimal(binary, &FLOAT32_LAYOUT, 20)
}

#[wasm_bindgen]
pub fn binary_to_float32_ext(binary: &str) -> BinaryInfo {
    binary_to_decimal_ext(binary, &FLOAT32_LAYOUT, 20)
}
