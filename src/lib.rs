use base64::decode;
use image::load_from_memory;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale is called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let img = load_from_memory(&base64_to_vector);
    log(&"Image loaded".into())
}
