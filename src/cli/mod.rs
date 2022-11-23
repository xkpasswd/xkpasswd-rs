#[cfg(test)]
mod tests;

use crate::bit_flags::*;
use crate::prelude::*;

use clap::Parser;

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
