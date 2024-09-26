use wasm_bindgen::prelude::*;

// This will expose the function to JavaScript
#[wasm_bindgen]
pub fn float32_to_binary(float: &str) -> String {
    let bits = float.parse::<f32>().unwrap().to_bits();
    format!("{:032b}", bits)
}

// An example of converting binary to a 32-bit float
#[wasm_bindgen]
pub fn binary_to_float32(binary: &str) -> Result<f32, JsValue> {
    if binary.len() != 32 {
        return Err(JsValue::from_str("Binary string must be 32 bits long."));
    }
    let bits = u32::from_str_radix(binary, 2)
        .map_err(|_| JsValue::from_str("Invalid binary string"))?;
    Ok(f32::from_bits(bits))
}
