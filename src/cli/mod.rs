#[cfg(test)]
mod tests;

use crate::bit_flags::*;
use crate::prelude::*;

use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(
        short = 'w',
        long = "words",
        help = "total number of words from dictionary"
    )]
    words_count: Option<u8>,

    #[arg(short = 'l', long = "word-min", help = "Minimum length of a word")]
    word_length_min: Option<u8>,

    #[arg(short = 'u', long = "word-max", help = "Maximum length of a word")]
    word_length_max: Option<u8>,

    #[arg(
        short = 't',
        long = "transforms",
        value_enum,
        help = "Word transformations"
    )]
    word_transforms: Option<Vec<WordTransform>>,

    #[arg(
        short = 's',
        long = "separators",
        help = "List of characters to be used as separator"
    )]
    separators: Option<String>,

    #[arg(
        long = "digits-before",
        help = "How many digits to be padded before the words"
    )]
    padding_digits_before: Option<u8>,

    #[arg(
        long = "digits-after",
        help = "How many digits to be padded after the words"
    )]
    padding_digits_after: Option<u8>,

    #[arg(
        short = 'y',
        long = "symbols",
        help = "List of characters to be used as padding symbols"
    )]
    padding_symbols: Option<String>,

    #[arg(
        long = "symbols-before",
        help = "How many symbols to be padded before the words"
    )]
    padding_symbols_before: Option<u8>,

    #[arg(
        long = "symbols-after",
        help = "How many symbols to be padded after the words"
    )]
    padding_symbols_after: Option<u8>,

    #[arg(short = 'f', long = "no-padding", help = "No extra symbols padding")]
    fixed_padding: bool,

    #[arg(
        short = 'a',
        long = "adaptive",
        help = "Pad or trim the final output to fit a length"
    )]
    adaptive_padding: Option<u8>,

    #[arg(short = 'p', long = "preset", value_enum)]
    preset: Option<Preset>,

    #[arg(short, long = "verbose", help = "Verbosity: 1 = info, 2+ = debug", action = ArgAction::Count)]
    verbosity: u8,
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

        self.init_logger();
        Ok(settings)
    }

    #[cfg(test)]
    fn init_logger(&self) {}

    #[cfg(not(test))]
    fn init_logger(&self) {
        stderrlog::new()
            .module("xkpasswd")
            .quiet(self.verbosity == 0)
            .show_level(false)
            .timestamp(stderrlog::Timestamp::Off)
            .verbosity((self.verbosity + 1) as usize)
            .init()
            .unwrap();
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
            Self::AppleID => PossibleValue::new("apple-id").help("Apple ID passwords"),
            Self::WindowsNtlmV1 => PossibleValue::new("ntlm").help("Windows NTLM v1"),
            Self::SecurityQuestions => PossibleValue::new("secq").help("Security questions"),
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
            Self::Lowercase => PossibleValue::new("lowercase").help(self.to_string()),
            Self::Titlecase => PossibleValue::new("titlecase").help(self.to_string()),
            Self::Uppercase => PossibleValue::new("uppercase").help(self.to_string()),
            Self::InversedTitlecase => {
                PossibleValue::new("inversed-titlecase").help(self.to_string())
            }
            Self::AltercaseLowerFirst => {
                PossibleValue::new("altercase-lower-first").help(self.to_string())
            }
            Self::AltercaseUpperFirst => {
                PossibleValue::new("altercase-upper-first").help(self.to_string())
            }
        })
    }
}