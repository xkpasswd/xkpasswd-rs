mod bit_flags;
mod cli;
mod prelude;
mod settings;

use cli::*;
use prelude::*;
use settings::*;

fn main() {
    let settings: Settings = Cli::parse_and_build_settings();
    log::info!("generating password with {}", settings);

    let pass_generator = Xkpasswd::default();
    let (passwd, entropy) = pass_generator.gen_pass(&settings);
    log::info!("calculated entropy: {}", entropy);

    println!("{}", passwd);
}
