//#![feature(test)]

pub mod prelude;
pub mod settings;

use prelude::*;
use settings::*;
use wasm_bindgen::prelude::*;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct Xkpasswd {
    dict: Dict<'static>,
}

#[wasm_bindgen]
impl Xkpasswd {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Xkpasswd {
        set_panic_hook();
        let dict_en_bytes = include_bytes!("./assets/dict_en.txt");
        let dict = load_dict(&dict_en_bytes[..]);
        Xkpasswd { dict }
    }

    pub fn gen_pass(&self, js_settings: JsValue) -> String {
        let settings: Settings =
            serde_wasm_bindgen::from_value(js_settings).expect("Invalid settings");

        log!("Settings: {:?}", settings);
        gen_passwd(&self.dict, &settings)
    }
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_gen_passwd() {
        let pass = Xkpasswd::new();

        let settings = &Settings::default().words_count(3).word_lengths(5, 8);
        let js_settings = serde_wasm_bindgen::to_value(settings).unwrap();
        assert_eq!(4, pass.gen_pass(js_settings).split('.').count());
    }
}
