mod bit_flags;
mod cli;
mod prelude;
mod settings;

use cli::*;
use prelude::*;
use settings::*;

fn main() {
    let mut cli = Cli::init();
    let settings: Settings = cli.parse_settings();
    let language = cli.language();
    log::info!("generating password in {:?} with {}", language, settings);

    let pass_generator = Xkpasswd::for_language(language);
    let (passwd, entropy) = pass_generator.gen_pass(&settings);
    log::info!("calculated entropy: {}", entropy);

    println!("{}", passwd);
}
