//#![feature(test)]

mod prelude;
pub mod settings;
mod wasm_utils;

use prelude::*;
use settings::*;
use wasm_bindgen::prelude::*;
use wasm_utils::*;

#[wasm_bindgen(js_name = "Settings")]
#[derive(Debug, Default)]
pub struct WasmSettings {
    settings: Settings,
}

#[wasm_bindgen(js_class = "Settings")]
impl WasmSettings {
    #[wasm_bindgen]
    pub fn default() -> WasmSettings {
        let settings = Settings::default();
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "setWordsCount")]
    pub fn set_words_count(&self, words_count: u8) -> WasmSettings {
        let settings = self.settings.words_count(words_count);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "setWordLengths")]
    pub fn set_word_length(&self, min: u8, max: u8) -> WasmSettings {
        let settings = self.settings.word_lengths(min, max);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "setSeparators")]
    pub fn set_separators(&self, separators: &str) -> WasmSettings {
        let settings = self.settings.separators(separators);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "setPaddingDigits")]
    pub fn set_padding_digits(&self, prefix: u8, suffix: u8) -> WasmSettings {
        let settings = self.settings.padding_digits(prefix, suffix);
        WasmSettings { settings }
    }
}

#[wasm_bindgen(js_name = "Xkpasswd")]
#[derive(Debug, Default)]
pub struct WasmXkpasswd {
    pass_generator: Xkpasswd,
}

#[wasm_bindgen(js_class = "Xkpasswd")]
impl WasmXkpasswd {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmXkpasswd {
        set_panic_hook();
        let pass_generator = Xkpasswd::new();
        WasmXkpasswd { pass_generator }
    }

    #[wasm_bindgen(js_name = "genPass")]
    pub fn gen_pass(&self, js_settings: &WasmSettings) -> String {
        let settings: Settings = js_settings.settings.clone();

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
        let pass = WasmXkpasswd::new();

        let settings = Settings::default()
            .words_count(3)
            .word_lengths(5, 8)
            .separators(".")
            .padding_digits(0, 2);
        let js_settings = &WasmSettings { settings };
        assert_eq!(4, pass.gen_pass(js_settings).split('.').count());
    }
}
