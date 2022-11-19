pub mod bit_flags;
pub mod prelude;
pub mod settings;

use bit_flags::*;
use prelude::*;
use settings::*;

fn main() {
    let pass_generator = Xkpasswd::new();
    let settings: Settings = build_settings(None).expect("Invalid settings");
    println!("Custom: {}", pass_generator.gen_pass(&settings));

    for preset in [
        Preset::AppleID,
        Preset::Default,
        Preset::WindowsNTLMv1,
        Preset::SecurityQuestions,
        Preset::Web16,
        Preset::Web32,
        Preset::Wifi,
        Preset::XKCD,
    ] {
        let settings: Settings = build_settings(Some(preset)).unwrap();
        println!("{:?}: {}", preset, pass_generator.gen_pass(&settings));
    }
}

fn build_settings<B: Builder + Randomizer>(
    optional_preset: Option<Preset>,
) -> Result<B, &'static str> {
    if let Some(preset) = optional_preset {
        return Ok(B::from_preset(preset));
    }

    B::default()
        .with_words_count(3)?
        .with_word_lengths(4, 8)?
        .with_word_transforms(WordTransform::Lowercase | WordTransform::Uppercase)?
        .with_separators(".")
        .with_padding_digits(0, 2)
        .with_padding_symbols("!@#$%^&*-_=+:|~?/;")
        .with_padding_symbol_lengths(0, 2)
        .with_padding_strategy(PaddingStrategy::Fixed)
}
