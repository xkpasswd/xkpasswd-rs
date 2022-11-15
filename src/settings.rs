use std::collections::HashMap;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

const DEFAULT_PADDING_LENGTH: u8 = 2;
const DEFAULT_SEPARATORS: &str = " .-_~";
const DEFAULT_SYMBOLS: &str = "!@#$%^&*-_=+:|~?/;";
const DEFAULT_WORDS_COUNT: u8 = 3;
const DEFAULT_WORD_LENGTHS: (u8, u8) = (4, 10);

#[derive(Clone, Debug)]
pub struct Settings {
    words_count: u8,
    word_lengths: (u8, u8),
    separators: String,
    padding_digits: (u8, u8),
    padding_symbols: String,
    padding_symbol_lengths: (u8, u8),
}

pub trait Builder {
    fn with_words_count(&self, words_count: u8) -> Self;
    fn with_word_lengths(&self, min_length: u8, max_length: u8) -> Self;
    fn with_separators(&self, separators: &str) -> Self;
    fn with_padding_digits(&self, prefix: u8, suffix: u8) -> Self;
    fn with_padding_symbols(&self, symbols: &str) -> Self;
    fn with_padding_symbol_lengths(&self, prefix: u8, suffix: u8) -> Self;
}

pub trait Randomizer {
    fn iter_word_lengths<F: FnMut(u8)>(&self, callback: F);
    fn rand_words(&self, words: &[&str]) -> Vec<String>;
    fn rand_separator(&self) -> String;
    fn rand_prefix(&self) -> String;
    fn rand_suffix(&self) -> String;
    fn transform_word(&self, word: &str) -> String;
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            words_count: DEFAULT_WORDS_COUNT,
            word_lengths: DEFAULT_WORD_LENGTHS,
            separators: DEFAULT_SEPARATORS.to_string(),
            padding_digits: (0, DEFAULT_PADDING_LENGTH),
            padding_symbols: DEFAULT_SYMBOLS.to_string(),
            padding_symbol_lengths: (0, DEFAULT_PADDING_LENGTH),
        }
    }
}

impl Builder for Settings {
    fn with_words_count(&self, words_count: u8) -> Settings {
        let mut cloned = self.clone();
        cloned.words_count = words_count;
        cloned
    }

    fn with_word_lengths(&self, min_length: u8, max_length: u8) -> Settings {
        let word_lengths = if min_length > max_length {
            (max_length, min_length)
        } else {
            (min_length, max_length)
        };

        let mut cloned = self.clone();
        cloned.word_lengths = word_lengths;
        cloned
    }

    fn with_separators(&self, separators: &str) -> Settings {
        let mut cloned = self.clone();
        cloned.separators = separators.to_string();
        cloned
    }

    fn with_padding_digits(&self, prefix: u8, suffix: u8) -> Settings {
        let mut cloned = self.clone();
        cloned.padding_digits = (prefix, suffix);
        cloned
    }

    fn with_padding_symbols(&self, symbols: &str) -> Settings {
        let mut cloned = self.clone();
        cloned.padding_symbols = symbols.to_string();
        cloned
    }

    fn with_padding_symbol_lengths(&self, prefix: u8, suffix: u8) -> Settings {
        let mut cloned = self.clone();
        cloned.padding_symbol_lengths = (prefix, suffix);
        cloned
    }
}

impl Randomizer for Settings {
    fn rand_words(&self, words: &[&str]) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let mut index_marker: HashMap<usize, bool> = HashMap::new();
        let word_indices = Uniform::from(0..words.len());
        (0..self.words_count)
            .map(|_| loop {
                let index: usize = word_indices.sample(&mut rng);
                let word = words[index];

                if index_marker.get(&index).is_none() {
                    index_marker.insert(index, true);
                    break self.transform_word(word);
                }
            })
            .collect()
    }

    fn rand_separator(&self) -> String {
        rand_chars(&self.separators, 1)
    }

    fn rand_prefix(&self) -> String {
        let (prefix_digits, _) = self.padding_digits;
        let (prefix_symbols, _) = self.padding_symbol_lengths;
        format!(
            "{}{}",
            rand_chars(&self.padding_symbols, prefix_symbols),
            rand_digits(prefix_digits)
        )
    }

    fn rand_suffix(&self) -> String {
        let (_, suffix_digits) = self.padding_digits;
        let (_, suffix_symbols) = self.padding_symbol_lengths;
        format!(
            "{}{}",
            rand_digits(suffix_digits),
            rand_chars(&self.padding_symbols, suffix_symbols)
        )
    }

    fn iter_word_lengths<F: FnMut(u8)>(&self, callback: F) {
        let (min, max) = self.word_lengths;
        (min..(max + 1)).for_each(callback);
    }

    fn transform_word(&self, word: &str) -> String {
        let mut rng = rand::thread_rng();

        if rng.gen::<bool>() {
            word.to_uppercase()
        } else {
            word.to_string()
        }
    }
}

impl Settings {
    pub const WORD_TRANSFORM_LOWERCASE: u8 = 0b001;
    pub const WORD_TRANSFORM_TITLECASE: u8 = 0b010;
    pub const WORD_TRANSFORM_UPPERCASE: u8 = 0b100;
}

fn rand_digits(count: u8) -> String {
    if count == 0 {
        return "".to_string();
    }

    let affordable_count = 20u32.min(count as u32);

    let lower_bound = 10u64.pow(affordable_count - 1);
    let upper_bound = if affordable_count == 20 {
        u64::MAX
    } else {
        10u64.pow(affordable_count)
    };

    let mut rng = rand::thread_rng();
    let padding_digits: u64 = Uniform::from(lower_bound..upper_bound).sample(&mut rng);
    padding_digits.to_string()
}

fn rand_chars(pool: &str, count: u8) -> String {
    if pool.is_empty() {
        return "".to_string();
    }

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..pool.len());
    pool.chars()
        .nth(idx)
        .unwrap()
        .to_string()
        .repeat(count as _)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();

        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
    }

    #[test]
    fn test_with_words_count() {
        let settings = Settings::default().with_words_count(1);
        // only words_count updated
        assert_eq!(1, settings.words_count);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);

        // overriding with multiple calls
        let other_settings = settings.with_words_count(123);
        assert_eq!(123, other_settings.words_count);
    }

    #[test]
    fn test_with_word_lengths() {
        let settings = Settings::default().with_word_lengths(2, 3);
        // only word_lengths updated
        assert_eq!((2, 3), settings.word_lengths);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);

        // overriding with multiple calls
        let other_settings = settings.with_word_lengths(5, 5);
        assert_eq!((5, 5), other_settings.word_lengths); // equal values

        let other_settings = settings.with_word_lengths(6, 4);
        assert_eq!((4, 6), other_settings.word_lengths); // min/max corrected
    }

    #[test]
    fn test_with_separators() {
        let settings = Settings::default().with_separators("abc123");
        // only separators updated
        assert_eq!("abc123".to_string(), settings.separators);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);

        // overriding with multiple calls
        let other_settings = settings.with_separators("");
        assert_eq!("".to_string(), other_settings.separators);
    }

    #[test]
    fn test_with_padding_digits() {
        let settings = Settings::default().with_padding_digits(1, 3);
        // only padding_digits updated
        assert_eq!((1, 3), settings.padding_digits);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);

        // overriding with multiple calls
        let other_settings = settings.with_padding_digits(0, 0);
        assert_eq!((0, 0), other_settings.padding_digits);
    }

    #[test]
    fn test_with_padding_symbols() {
        let settings = Settings::default().with_padding_symbols("456xyz");
        // only padding_symbols updated
        assert_eq!("456xyz".to_string(), settings.padding_symbols);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);

        // overriding with multiple calls
        let other_settings = settings.with_padding_digits(0, 0);
        assert_eq!((0, 0), other_settings.padding_digits);
    }

    #[test]
    fn test_rand_prefix() {
        let empty_cases = [
            ((0, 0), (0, 0)),
            ((0, 1), (0, 0)),
            ((0, 0), (0, 2)),
            ((0, 3), (0, 4)),
        ];

        for ((prefix_digits, suffix_digits), (prefix_symbols, suffix_symbols)) in empty_cases {
            let settings = Settings::default()
                .with_padding_digits(prefix_digits, suffix_digits)
                .with_padding_symbol_lengths(prefix_symbols, suffix_symbols);
            assert_eq!("", settings.rand_prefix());
        }

        for prefix_symbols in 1usize..10 {
            for prefix_digits in 1usize..10 {
                let settings = Settings::default()
                    .with_padding_digits(prefix_digits as u8, 2)
                    .with_padding_symbols("#")
                    .with_padding_symbol_lengths(prefix_symbols as u8, 3);
                let prefix = settings.rand_prefix();

                // total length of prefix
                assert_eq!(prefix_symbols + prefix_digits, prefix.len());

                // first partition [0..prefix_symbols] is the repeated symbol
                assert_eq!(
                    "#".to_string().repeat(prefix_symbols),
                    &prefix[..prefix_symbols]
                );

                // second partition [prefix_symbols..prefix_symbols+prefix_digits]
                // is the stringified digits
                let _ = &prefix[prefix_symbols..].parse::<u64>().unwrap();
            }
        }
    }

    #[test]
    fn test_rand_suffix() {
        let empty_cases = [
            ((0, 0), (0, 0)),
            ((1, 0), (0, 0)),
            ((0, 0), (2, 0)),
            ((3, 0), (4, 0)),
        ];

        for ((prefix_digits, suffix_digits), (prefix_symbols, suffix_symbols)) in empty_cases {
            let settings = Settings::default()
                .with_padding_digits(prefix_digits, suffix_digits)
                .with_padding_symbol_lengths(prefix_symbols, suffix_symbols);
            assert_eq!("", settings.rand_suffix());
        }

        for suffix_symbols in 1usize..10 {
            for suffix_digits in 1usize..10 {
                let settings = Settings::default()
                    .with_padding_digits(2, suffix_digits as u8)
                    .with_padding_symbols("~")
                    .with_padding_symbol_lengths(3, suffix_symbols as u8);
                let suffix = settings.rand_suffix();

                // total length of suffix
                assert_eq!(suffix_digits + suffix_symbols, suffix.len());

                // first partition [0..suffix_digits] is the stringified digits
                let _ = &suffix[..suffix_digits].parse::<u64>().unwrap();

                // second partition [suffix_digits..suffix_digits+suffix_symbols]
                // is repeated symbols
                assert_eq!(
                    "~".to_string().repeat(suffix_symbols),
                    &suffix[suffix_digits..]
                );
            }
        }
    }

    #[test]
    fn test_rand_digits() {
        assert_eq!("", rand_digits(0));

        for count in 1..21 {
            for _ in 0..100 {
                let digits = rand_digits(count);
                assert_eq!(count as usize, digits.len());
            }
        }

        for count in 21..100 {
            for _ in 0..100 {
                let digits = rand_digits(count);
                assert_eq!(20, digits.len());
            }
        }
    }

    #[test]
    fn test_rand_chars() {
        assert_eq!("".to_string(), rand_chars("", 1));

        // single char randomize
        for _ in 0..10 {
            let result = rand_chars(DEFAULT_SYMBOLS, 1);
            assert!(DEFAULT_SYMBOLS.contains(&result));
        }

        // multi char randomize
        for _ in 0..10 {
            for count in 2..5 {
                let result = rand_chars("$", count);
                assert_eq!(count as usize, result.len());
                assert_eq!("$".to_string().repeat(count as usize), result);
            }
        }
    }
}
