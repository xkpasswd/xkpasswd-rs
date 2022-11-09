//#![feature(test)]

pub mod prelude;
pub mod settings;

use settings::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_pass(settings: &Settings) -> String {
    set_panic_hook();
    prelude::gen_passwd(settings)
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
        let settings = &Settings { words_count: 3 };
        assert_eq!(4, gen_pass(settings).split('.').count());
    }
}
