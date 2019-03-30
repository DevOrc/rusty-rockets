use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    log("Hello from Rust!");

    Ok(())
}

#[wasm_bindgen]
pub fn add_with_rust(num1: i32, num2: i32) -> i32 {
    num1 + num2
}