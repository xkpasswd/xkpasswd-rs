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
pub fn gen_pass(js_settings: JsValue) -> String {
    set_panic_hook();

    log!("js_settings: {:?}", js_settings);
    let settings: Settings = serde_wasm_bindgen::from_value(js_settings).expect("Invalid settings");
    log!("settings: {:?}", settings);

    gen_passwd(&settings)
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
        let settings = &Settings::default().words_count(3).word_lengths(5, 8);
        let js_settings = serde_wasm_bindgen::to_value(settings).unwrap();
        assert_eq!(4, gen_pass(js_settings).split('.').count());
    }
}
