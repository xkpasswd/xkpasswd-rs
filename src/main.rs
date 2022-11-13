mod prelude;
pub mod settings;

use prelude::*;
use settings::*;

fn main() {
    let pass_generator = Xkpasswd::new();
    let settings = &Settings::default()
        .words_count(3)
        .word_lengths(5, 8)
        .separators("._~");
    println!("{}", pass_generator.gen_pass(settings));
}
