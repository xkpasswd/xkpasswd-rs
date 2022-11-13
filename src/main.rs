mod prelude;
pub mod settings;

use prelude::*;
use settings::*;

fn main() {
    let pass_generator = Xkpasswd::new();
    let settings = &Settings::default()
        .with_words_count(3)
        .with_word_lengths(5, 8)
        .with_separators("._~")
        .with_padding_digits(0, 2)
        .with_padding_symbols("~!@#$%^&*")
        .with_padding_symbol_lengths(0, 2);
    println!("{}", pass_generator.gen_pass(settings));
}
