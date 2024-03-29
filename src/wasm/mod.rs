mod utils;

use crate::prelude::*;
use crate::settings::*;
use wasm_bindgen::prelude::*;

use utils::*;

const DEFAULT_SETTING_BUILDER_ERR: &str = "Invalid settings";

#[wasm_bindgen(js_name = "Settings")]
#[derive(Debug, Default)]
pub struct WasmSettings {
    settings: Settings,
}

#[wasm_bindgen(js_class = "Settings")]
impl WasmSettings {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmSettings {
        WasmSettings::default()
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
    pub fn with_word_lengths(&self, min: Option<u8>, max: Option<u8>) -> WasmSettings {
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
    pub fn with_padding_digits(&self, prefix: Option<u8>, suffix: Option<u8>) -> WasmSettings {
        let settings = self.settings.with_padding_digits(prefix, suffix);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withPaddingSymbols")]
    pub fn with_padding_symbols(&self, symbols: &str) -> WasmSettings {
        let settings = self.settings.with_padding_symbols(symbols);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withPaddingSymbolLengths")]
    pub fn with_padding_symbol_lengths(
        &self,
        prefix: Option<u8>,
        suffix: Option<u8>,
    ) -> WasmSettings {
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
    pub fn with_adaptive_padding(&self, length: usize) -> WasmSettings {
        let settings = self
            .settings
            .with_padding_strategy(PaddingStrategy::Adaptive(length))
            .expect(DEFAULT_SETTING_BUILDER_ERR);
        WasmSettings { settings }
    }

    #[wasm_bindgen(js_name = "withWordTransforms")]
    pub fn with_word_transforms(&self, transforms: u8) -> WasmSettings {
        let settings = self
            .settings
            .with_word_transforms(transforms)
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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PasswdResult {
    passwd: String,
    pub entropy: Entropy,
}

#[wasm_bindgen]
impl PasswdResult {
    #[wasm_bindgen(getter)]
    pub fn passwd(&self) -> String {
        self.passwd.clone()
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
        WasmXkpasswd::default()
    }

    #[wasm_bindgen(js_name = "genPass")]
    pub fn gen_pass(&self, js_settings: &WasmSettings) -> PasswdResult {
        let settings: Settings = js_settings.settings.clone();

        let (passwd, entropy) = self.pass_generator.gen_pass(&settings);
        console_log!("{:?} {:?}", settings, entropy);

        PasswdResult { passwd, entropy }
    }
}
