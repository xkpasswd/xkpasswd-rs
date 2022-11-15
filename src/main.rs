mod prelude;
pub mod settings;

use prelude::*;
use settings::*;

fn main() {
    let pass_generator = Xkpasswd::new();
    let settings = Settings::default()
        .with_words_count(3)
        .with_word_lengths(4, 8)
        .with_separators(".")
        .with_padding_digits(0, 2)
        .with_padding_symbols("!@#$%^&*-_=+:|~?/;")
        .with_padding_symbol_lengths(0, 2)
        .with_word_transforms(WordTransform::LOWERCASE | WordTransform::UPPERCASE)
        .expect("Invalid settings");
    println!("{}", pass_generator.gen_pass(&settings));
}
