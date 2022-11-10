use xkpasswd::prelude::*;
use xkpasswd::settings::*;

fn main() {
    let settings = &Settings::default().words_count(3).word_lengths(5, 8);
    println!("{}", gen_passwd(settings));
}
