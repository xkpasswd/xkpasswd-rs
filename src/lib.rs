//#![feature(test)]

pub mod prelude;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_pass(count: u8) -> String {
    set_panic_hook();
    prelude::gen_passwd(count)
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
        assert_eq!(4, gen_pass(3).split('.').count());
    }
}
