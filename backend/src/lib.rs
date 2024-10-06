use crate::ieee754_ops::{binary_to_decimal, binary_to_decimal_ext, BinaryInfo, FLOAT32_LAYOUT};
use ieee754_ops::decimal_to_binary;
use wasm_bindgen::prelude::*;

mod bitfield;
mod ieee754_ops;
