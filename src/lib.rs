//! # xkpasswd
//!
//! XKCD-style password generator written in Rust with WASM support.
//!
//! Inspired by [XKCD #936](https://xkcd.com/936/) - "correct horse battery staple".
//!
//! ## Features
//!
//! - **CLI application** for generating secure, memorable passwords
//! - **WASM module** for web integration
//! - **Multiple language support**: English, German, Spanish, French, Portuguese
//! - **Configurable presets**: AppleID, Web16, Web32, WiFi, XKCD, and more
//! - **Entropy calculation** to help assess password strength
//!
//! ## Usage (Library)
//!
//! ```rust
//! use xkpasswd::prelude::{Xkpasswd, L10n, Language};
//! use xkpasswd::settings::Settings;
//!
//! let generator = Xkpasswd::for_language(Language::English);
//! let settings = Settings::default();
//! let (password, entropy) = generator.gen_pass(&settings);
//! println!("Password: {}, Entropy: {:.2}-{:.2} bits", password, entropy.blind_min, entropy.blind_max);
//! ```

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
        assert_eq!(4, pass.gen_pass(&settings).passwd().split('.').count());
    }
}
