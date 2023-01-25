use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(input: String) -> String  {
    console_error_panic_hook::set_once();

    input.to_uppercase()
}
