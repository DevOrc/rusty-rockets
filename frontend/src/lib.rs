use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/js/engine.js")]
extern {
    fn send_message(s: &str);

    fn initEngine();
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
    
    initEngine();
    draco::start(HelloWorld, draco::select("dracoApp").expect("dracoApp").into());
    send_message("Draco Started!");
    
    Ok(())
}