#[cfg(test)]
mod tests;

use crate::bit_flags::*;
use crate::prelude::*;

use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(short = 'w', long = "words")]
    words_count: Option<u8>,

    #[arg(long = "word-min")]
    word_length_min: Option<u8>,

    #[arg(long = "word-max")]
    word_length_max: Option<u8>,

    #[arg(short = 't', long, value_enum)]
    word_transforms: Option<Vec<WordTransform>>,

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
    preset: Option<Preset>,
}

impl Cli {
    pub fn build_settings<B: Builder + Randomizer>(&self) -> Result<B, &'static str> {
        let mut settings = if let Some(preset) = self.preset {
            B::from_preset(preset)
        } else {
            B::default()
        };

        settings = settings
            .with_word_lengths(self.word_length_min, self.word_length_max)?
            .with_padding_digits(self.padding_digits_before, self.padding_digits_after)
            .with_padding_symbol_lengths(self.padding_symbols_before, self.padding_symbols_after);

        if let Some(words_count) = self.words_count {
            settings = settings.with_words_count(words_count)?
        }

        if let Some(word_transforms) = &self.word_transforms {
            let transforms: FieldSize = word_transforms
                .iter()
                .fold(0 as FieldSize, |acc, cur| acc | *cur);
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

impl ValueEnum for Preset {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Default,
            Self::AppleID,
            Self::WindowsNtlmV1,
            Self::SecurityQuestions,
            Self::Web16,
            Self::Web32,
            Self::Wifi,
            Self::Xkcd,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Default => PossibleValue::new("default"),
            Self::AppleID => PossibleValue::new("apple-id").help("Apple ID password"),
            Self::WindowsNtlmV1 => PossibleValue::new("ntlm").help("Windows NTLM v1"),
            Self::SecurityQuestions => {
                PossibleValue::new("sec-questions").help("Security questions")
            }
            Self::Web16 => {
                PossibleValue::new("web16").help("Maxium 16 characters for older websites")
            }
            Self::Web32 => {
                PossibleValue::new("web32").help("Maximum 32 characters for modern websites")
            }
            Self::Wifi => PossibleValue::new("wifi").help("Fixed 63 characters for Wifi WPA2 keys"),
            Self::Xkcd => {
                PossibleValue::new("xkcd").help("As described in the original XKCD comic")
            }
        })
    }
}

impl ValueEnum for WordTransform {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Lowercase,
            Self::Titlecase,
            Self::Uppercase,
            Self::InversedTitlecase,
            Self::AltercaseLowerFirst,
            Self::AltercaseUpperFirst,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Lowercase => PossibleValue::new("lowercase").help("lowercase"),
            Self::Titlecase => PossibleValue::new("titlecase").help("Titlecase"),
            Self::Uppercase => PossibleValue::new("uppercase").help("UPPERCASE"),
            Self::InversedTitlecase => {
                PossibleValue::new("inversed-titlecase").help("iNVERSED tITLECASE")
            }
            Self::AltercaseLowerFirst => {
                PossibleValue::new("altercase-lower-first").help("altercase LOWER first")
            }
            Self::AltercaseUpperFirst => {
                PossibleValue::new("altercase-upper-first").help("ALTERCASE upper FIRST")
            }
        })
    }
}
