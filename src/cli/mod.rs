#[cfg(test)]
mod tests;
mod toml_conf;

use crate::bit_flags::*;
use crate::prelude::*;
use toml_conf::*;

use clap::builder::PossibleValue;
use clap::error::ErrorKind;
use clap::{ArgAction, CommandFactory, Parser, ValueEnum};

#[derive(Clone, Copy, Debug)]
pub enum CliPadding {
    Fixed,
    Adaptive,
}

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
        help = "Word transformations, can be combined with multiple occurrences"
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

    #[arg(short = 'p', long = "padding", help = "Padding strategy", value_enum)]
    padding: Option<CliPadding>,

    #[arg(
        short = 'a',
        long = "adaptive-length",
        help = "Pad or trim the final output to fit a length. Required for --padding=adaptive"
    )]
    adaptive_length: Option<usize>,

    #[arg(short = 'P', long = "preset", value_enum)]
    preset: Option<Preset>,

    #[arg(short = 'v', long = "verbose", help = "Verbosity: 1 = info, 2+ = debug", action = ArgAction::Count)]
    verbosity: u8,

    #[arg(short = 'c', long = "config", help = "Path to .toml config file")]
    config_file: Option<String>,
}

impl Cli {
    pub fn parse_and_build<B: Builder + Randomizer>() -> B {
        let mut cli = Self::parse();
        cli.init_logger();

        let settings_builder = move |cli: Self| match cli.build_settings::<B>() {
            Err(err) => Err(format!("Invalid settings: {}", err)),
            Ok(settings) => Ok(settings),
        };

        let result = match cli.parse_config_file() {
            Ok(_) => settings_builder(cli),
            Err(err) => match err {
                ConfigParseError::Ignore => settings_builder(cli),
                ConfigParseError::InvalidFile(err) => {
                    Err(format!("Error parsing config file: {}", err))
                }
                ConfigParseError::InvalidConfig(field, err) => {
                    Err(format!("Error parsing config file at '{}': {}", field, err))
                }
            },
        };

        match result {
            Ok(settings) => settings,
            Err(message) => {
                Self::command()
                    .error(ErrorKind::InvalidValue, message)
                    .exit();
            }
        }
    }

    fn build_settings<B: Builder + Randomizer>(&self) -> Result<B, String> {
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

        if let Some(padding) = &self.padding {
            match padding {
                CliPadding::Fixed => {
                    settings = settings.with_padding_strategy(PaddingStrategy::Fixed)?
                }
                CliPadding::Adaptive => {
                    if let Some(adaptive_length) = &self.adaptive_length {
                        settings = settings
                            .with_padding_strategy(PaddingStrategy::Adaptive(*adaptive_length))?
                    } else {
                        return Err(
                            "adaptive length is required for adaptive padding strategy".to_string()
                        );
                    }
                }
            }
        }

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
            .show_module_names(false)
            .timestamp(stderrlog::Timestamp::Off)
            .verbosity((self.verbosity + 1) as usize)
            .init()
            .unwrap();
    }
}

impl ValueEnum for CliPadding {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Fixed, Self::Adaptive]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Fixed => PossibleValue::new("fixed")
                .help("Fixed numbers of symbols to be padded before & after words"),
            Self::Adaptive => PossibleValue::new("adaptive").help(
                r#"Pad or trim the final output to fit a length. Requires --adaptive-length.
Notes: setting this will disable --symbols-before and --symbols-after options"#,
            ),
        })
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
            Self::Default => PossibleValue::new("default").help("Some sensible default values"),
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
