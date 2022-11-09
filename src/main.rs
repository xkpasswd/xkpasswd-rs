use xkpasswd::prelude::*;
use xkpasswd::settings::*;

fn main() {
    let settings = &Settings { words_count: 3 };
    println!("{}", gen_passwd(settings));
}
