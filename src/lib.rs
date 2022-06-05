use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    bare_bones();
    // using_a_macro();
    // using_web_sys();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn bare_bones() {
    log("Hello fron rust");
}