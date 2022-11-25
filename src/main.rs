mod bit_flags;
mod cli;
mod prelude;
mod settings;

use cli::*;
use prelude::*;
use settings::*;

use clap::Parser;

const DEFAULT_SETTING_BUILDER_ERR: &str = "Invalid settings";

fn main() {
    let cli = Cli::parse();
    let settings: Settings = cli.build_settings().expect(DEFAULT_SETTING_BUILDER_ERR);
    log::info!("generating password with {}", settings);

    let pass_generator = Xkpasswd::default();
    let (passwd, entropy) = pass_generator.gen_pass(&settings);
    log::info!("calculated entropy: {}", entropy);

    println!("{}", passwd);
}
