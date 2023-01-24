use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(input: String) -> Result<String, String> {
    Ok(input.to_uppercase())
}
