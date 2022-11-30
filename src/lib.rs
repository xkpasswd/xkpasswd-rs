pub mod bit_flags;
pub mod prelude;
pub mod settings;
mod wasm;

#[cfg(test)]
mod tests {
    use super::bit_flags::*;
    use super::wasm::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_gen_passwd() {
        let pass = WasmXkpasswd::new();

        let settings = WasmSettings::default()
            .with_words_count(3)
            .with_word_lengths(None, Some(8))
            .with_separators(".")
            .with_padding_digits(None, Some(2))
            .with_padding_symbols("!@#$%^&*-_=+:|~?/;")
            .with_padding_symbol_lengths(None, Some(2))
            .with_word_transforms(WordTransform::Lowercase | WordTransform::Uppercase)
            .with_fixed_padding();
        assert_eq!(4, pass.gen_pass(&settings).split('.').count());
    }
}
