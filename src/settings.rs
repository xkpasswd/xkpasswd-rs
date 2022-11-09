use wasm_bindgen::prelude::*;

pub const PADDING_SYMBOLS: &str = "!@#$%^&*-_=+:|~?/;";

#[wasm_bindgen]
pub struct Settings {
    pub words_count: u8,
}

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(constructor)]
    pub fn new(words_count: u8) -> Settings {
        Settings { words_count }
    }
}
