use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct HelloWorld;

impl draco::App for HelloWorld {
    type Message = ();

    fn render(&self) -> draco::Node<Self::Message> {
        draco::html::h1().push("This is a thing from Rust").into()
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log("web assembly started!");
    draco::start(HelloWorld, draco::select("dracoApp").expect("dracoApp").into());
    
    Ok(())
}