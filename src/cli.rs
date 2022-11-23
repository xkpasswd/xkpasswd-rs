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
            Self::Default => Preset::Default,
            Self::AppleID => Preset::AppleID,
            Self::WindowsNtlmV1 => Preset::WindowsNTLMv1,
            Self::SecurityQuestions => Preset::SecurityQuestions,
            Self::Web16 => Preset::Web16,
            Self::Web32 => Preset::Web32,
            Self::Wifi => Preset::Wifi,
            Self::Xkcd => Preset::Xkcd,
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CliTransform {
    // single transforms - possible to combine with each other
    Lowercase,
    Titlecase,
    Uppercase,
    InversedTitlecase,

    // group transforms - overriding other single ones
    AltercaseLowerFirst,
    AltercaseUpperFirst,
}

impl CliTransform {
    fn to_word_transform(self) -> WordTransform {
        match self {
            Self::Lowercase => WordTransform::Lowercase,
            Self::Titlecase => WordTransform::Titlecase,
            Self::Uppercase => WordTransform::Uppercase,
            Self::InversedTitlecase => WordTransform::InversedTitlecase,
            Self::AltercaseLowerFirst => WordTransform::AltercaseLowerFirst,
            Self::AltercaseUpperFirst => WordTransform::AltercaseUpperFirst,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(short = 'w', long = "words", default_value_t = 3)]
    words_count: u8,

    #[arg(long = "word-min")]
    word_length_min: Option<u8>,

    #[arg(long = "word-max")]
    word_length_max: Option<u8>,

    #[arg(short = 't', long, value_enum)]
    word_transforms: Option<Vec<CliTransform>>,

    #[arg(short = 's', long)]
    separators: Option<String>,

    #[arg(long = "digits-before")]
    padding_digits_before: Option<u8>,

    #[arg(long = "digits-after")]
    padding_digits_after: Option<u8>,

    #[arg(short = 'y', long = "symbols")]
    padding_symbols: Option<String>,

    #[arg(long = "symbols-before")]
    padding_symbols_before: Option<u8>,

    #[arg(long = "symbols-after")]
    padding_symbols_after: Option<u8>,

    #[arg(short = 'f', long)]
    fixed_padding: bool,

    #[arg(short = 'a', long)]
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
            .with_word_lengths(self.word_length_min, self.word_length_max)?
            .with_word_transforms(WordTransform::Lowercase | WordTransform::Uppercase)?
            .with_padding_digits(self.padding_digits_before, self.padding_digits_after)
            .with_padding_symbol_lengths(self.padding_symbols_before, self.padding_symbols_after);

        if let Some(cli_transforms) = &self.word_transforms {
            let transforms: FieldSize = cli_transforms
                .iter()
                .fold(0 as FieldSize, |acc, cur| acc | (*cur).to_word_transform());
            settings = settings.with_word_transforms(transforms)?;
        }

        if let Some(separators) = &self.separators {
            settings = settings.with_separators(separators);
        }

        if let Some(padding_symbols) = &self.padding_symbols {
            settings = settings.with_padding_symbols(padding_symbols);
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
