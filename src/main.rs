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
    let pass_generator = Xkpasswd::new();
    println!("{}", pass_generator.gen_pass(&settings));
}
