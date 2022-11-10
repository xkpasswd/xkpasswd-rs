use xkpasswd::prelude::*;
use xkpasswd::settings::*;

fn main() {
    let dict_en_bytes = include_bytes!("./assets/dict_en.txt");
    let dict = &load_dict(&dict_en_bytes[..]);
    let settings = &Settings::default().words_count(3).word_lengths(5, 8);
    println!("{}", gen_passwd(dict, settings));
}
