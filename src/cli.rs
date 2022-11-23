use crate::bit_flags::*;
use crate::prelude::*;

use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliPreset {
    Default,
    AppleID,
    WindowsNtlmV1,
    SecurityQuestions,
    Web16,
    Web32,
    Wifi,
    Xkcd,
}

impl CliPreset {
    fn to_preset(self) -> Preset {
        match self {
            CliPreset::Default => Preset::Default,
            CliPreset::AppleID => Preset::AppleID,
            CliPreset::WindowsNtlmV1 => Preset::WindowsNTLMv1,
            CliPreset::SecurityQuestions => Preset::SecurityQuestions,
            CliPreset::Web16 => Preset::Web16,
            CliPreset::Web32 => Preset::Web32,
            CliPreset::Wifi => Preset::Wifi,
            CliPreset::Xkcd => Preset::Xkcd,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(short = 'w', long = "words", default_value_t = 3)]
    words_count: u8,

    #[arg(short = 'l', long = "min", default_value_t = 4)]
    words_length_min: u8,

    #[arg(short = 'u', long = "max", default_value_t = 8)]
    words_length_max: u8,

    #[arg(short = 't', long, value_enum)]
    word_transforms: Option<Vec<CliPreset>>,

    #[arg(short, long)]
    separators: Option<String>,

    #[arg(long = "digits-before")]
    padding_digits_before: Option<u8>,

    #[arg(long = "digits-after")]
    padding_digits_after: Option<u8>,

    #[arg(long = "padding-symbols")]
    padding_symbols: Option<String>,

    #[arg(long = "symbols-before")]
    padding_symbols_before: Option<u8>,

    #[arg(long = "symbols-after")]
    padding_symbols_after: Option<u8>,

    #[arg(short, long)]
    fixed_padding: bool,

    #[arg(short, long)]
    adaptive_padding: Option<u8>,

    #[arg(short = 'p', long = "preset", value_enum)]
    preset: Option<CliPreset>,
}

impl Cli {
    pub fn build_settings<B: Builder + Randomizer>(&self) -> Result<B, &'static str> {
        let mut settings = if let Some(cli_preset) = self.preset {
            B::from_preset(cli_preset.to_preset())
        } else {
            B::default()
        };

        settings = settings
            .with_words_count(self.words_count)?
            .with_word_lengths(self.words_length_min, self.words_length_max)?
            .with_word_transforms(WordTransform::Lowercase | WordTransform::Uppercase)?;

        if let Some(separators) = &self.separators {
            settings = settings.with_separators(separators);
        }

        if self.fixed_padding {
            settings = settings.with_padding_strategy(PaddingStrategy::Fixed)?;
        }

        if let Some(adaptive_padding) = &self.adaptive_padding {
            settings =
                settings.with_padding_strategy(PaddingStrategy::Adaptive(*adaptive_padding))?
        }

        Ok(settings)
    }
}
