//#![feature(test)]

pub mod prelude;
pub mod settings;
mod wasm_utils;

use prelude::*;
use settings::*;
use wasm_bindgen::prelude::*;
use wasm_utils::*;

const DEFAULT_SETTING_BUILDER_ERR: &str = "Invalid settings";

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

    #[wasm_bindgen(js_name = "withWordsCount")]
    pub fn with_words_count(&self, words_count: u8) -> WasmSettings {
        let settings = self
            .settings
            .with_words_count(words_count)
            .expect(DEFAULT_SETTING_BUILDER_ERR);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withWordLengths")]
    pub fn with_word_lengths(&self, min: u8, max: u8) -> WasmSettings {
        let settings = self
            .settings
            .with_word_lengths(min, max)
            .expect(DEFAULT_SETTING_BUILDER_ERR);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withSeparators")]
    pub fn with_separators(&self, separators: &str) -> WasmSettings {
        let settings = self.settings.with_separators(separators);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withPaddingDigits")]
    pub fn with_padding_digits(&self, prefix: u8, suffix: u8) -> WasmSettings {
        let settings = self.settings.with_padding_digits(prefix, suffix);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withPaddingSymbols")]
    pub fn with_padding_symbols(&self, symbols: &str) -> WasmSettings {
        let settings = self.settings.with_padding_symbols(symbols);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withPaddingSymbolLengths")]
    pub fn with_padding_symbol_lengths(&self, prefix: u8, suffix: u8) -> WasmSettings {
        let settings = self.settings.with_padding_symbol_lengths(prefix, suffix);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withFixedPadding")]
    pub fn with_fixed_padding(&self) -> WasmSettings {
        let settings = self
            .settings
            .with_padding_strategy(PaddingStrategy::Fixed)
            .expect(DEFAULT_SETTING_BUILDER_ERR);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withAdaptivePadding")]
    pub fn with_adaptive_padding(&self, length: u8) -> WasmSettings {
        let settings = self
            .settings
            .with_padding_strategy(PaddingStrategy::Adaptive(length))
            .expect(DEFAULT_SETTING_BUILDER_ERR);
        WasmSettings { settings }
    }

    #[wasm_bindgen(variadic, js_name = "withWordTransforms")]
    pub fn with_word_transforms(&self, transforms: &[u8]) -> WasmSettings {
        let reduced = transforms.iter().fold(0, |acc, cur| acc | cur);
        let settings = self
            .settings
            .with_word_transforms(reduced)
            .expect(DEFAULT_SETTING_BUILDER_ERR);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "fromPreset")]
    pub fn from_preset(preset: Preset) -> WasmSettings {
        WasmSettings {
            settings: Settings::from_preset(preset),
        }
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

        let settings = WasmSettings::default()
            .with_words_count(3)
            .with_word_lengths(4, 8)
            .with_separators(".")
            .with_padding_digits(0, 2)
            .with_padding_symbols("!@#$%^&*-_=+:|~?/;")
            .with_padding_symbol_lengths(0, 2)
            .with_word_transforms(&[WordTransform::LOWERCASE, WordTransform::UPPERCASE])
            .with_fixed_padding();
        assert_eq!(4, pass.gen_pass(&settings).split('.').count());
    }
}
