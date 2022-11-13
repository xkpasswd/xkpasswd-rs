//#![feature(test)]

mod prelude;
pub mod settings;
mod wasm_utils;

use prelude::*;
use settings::*;
use wasm_bindgen::prelude::*;
use wasm_utils::*;

#[wasm_bindgen(js_name = "Xkpasswd")]
#[derive(Debug, Default)]
pub struct XkpasswdWasm {
    pass_generator: Xkpasswd,
}

#[wasm_bindgen(js_class = "Xkpasswd")]
impl XkpasswdWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> XkpasswdWasm {
        set_panic_hook();
        let pass_generator = Xkpasswd::new();
        XkpasswdWasm { pass_generator }
    }

    pub fn gen_pass(&self, js_settings: JsValue) -> String {
        let settings: Settings =
            serde_wasm_bindgen::from_value(js_settings).expect("Invalid settings");

        console_log!("Settings: {:?}", settings);
        self.pass_generator.gen_pass(&settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_gen_passwd() {
        let pass = XkpasswdWasm::new();

        let settings = &Settings::default()
            .words_count(3)
            .word_lengths(5, 8)
            .separators(".")
            .padding_digits(0, 2);
        let js_settings = serde_wasm_bindgen::to_value(settings).unwrap();
        assert_eq!(4, pass.gen_pass(js_settings).split('.').count());
    }
}
