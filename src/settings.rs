use super::bit_flags::{BitFlags, FieldSize, WordTransform};
use super::prelude::{Builder, Randomizer};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;
use std::result::Result;
use wasm_bindgen::prelude::*;

const MIN_WORD_LENGTH: u8 = 4;
const MIN_WORD_LENGTH_ERR: &str = "min word length must be 4 or higher";
const MAX_WORD_LENGTH: u8 = 10;
const MAX_WORD_LENGTH_ERR: &str = "max word length must be 10 or lower";
const DEFAULT_PADDING_LENGTH: u8 = 2;
const DEFAULT_SEPARATORS: &str = ".-_~";
const DEFAULT_SYMBOLS: &str = "~@$%^&*-_+=:|~?/.;";
const DEFAULT_WORDS_COUNT: u8 = 3;
const DEFAULT_WORD_LENGTHS: (u8, u8) = (MIN_WORD_LENGTH, MAX_WORD_LENGTH);
const DEFAULT_WORD_TRANSFORMS: FieldSize = 0b00000101; // WordTransform::Lowercase | WordTransform::Uppercase

#[derive(Clone, Debug)]
pub enum PaddingStrategy {
    Fixed,
    Adaptive(u8),
}

#[derive(Debug)]
pub enum PaddingResult {
    Unchanged,
    Trim(u8),
    Pad(String),
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Preset {
    AppleID,
    Default,
    WindowsNTLMv1,
    SecurityQuestions,
    Web16,
    Web32,
    Wifi,
    XKCD,
}

#[derive(Clone, Debug)]
pub struct Settings {
    words_count: u8,
    word_lengths: (u8, u8),
    word_transforms: u8,
    separators: String,
    padding_digits: (u8, u8),
    padding_symbols: String,
    padding_symbol_lengths: (u8, u8),
    padding_strategy: PaddingStrategy,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            words_count: DEFAULT_WORDS_COUNT,
            word_lengths: DEFAULT_WORD_LENGTHS,
            word_transforms: DEFAULT_WORD_TRANSFORMS,
            separators: DEFAULT_SEPARATORS.to_string(),
            padding_digits: (0, DEFAULT_PADDING_LENGTH),
            padding_symbols: DEFAULT_SYMBOLS.to_string(),
            padding_symbol_lengths: (0, DEFAULT_PADDING_LENGTH),
            padding_strategy: PaddingStrategy::Fixed,
        }
    }
}

impl Builder for Settings {
    fn with_words_count(&self, words_count: u8) -> Result<Self, &'static str> {
        if words_count == 0 {
            return Err("only positive integer is allowed for words count");
        }

        let mut cloned = self.clone();
        cloned.words_count = words_count;
        Ok(cloned)
    }

    fn with_word_lengths(&self, min_length: u8, max_length: u8) -> Result<Settings, &'static str> {
        let min = cmp::min(min_length, max_length);
        let max = cmp::max(min_length, max_length);

        if min < MIN_WORD_LENGTH {
            return Err(MIN_WORD_LENGTH_ERR);
        }

        if max > MAX_WORD_LENGTH {
            return Err(MAX_WORD_LENGTH_ERR);
        }

        let mut cloned = self.clone();
        cloned.word_lengths = (min, max);
        Ok(cloned)
    }

    fn with_separators(&self, separators: &str) -> Self {
        let mut cloned = self.clone();
        cloned.separators = separators.to_string();
        cloned
    }

    fn with_padding_digits(&self, prefix: u8, suffix: u8) -> Self {
        let mut cloned = self.clone();
        cloned.padding_digits = (prefix, suffix);
        cloned
    }

    fn with_padding_symbols(&self, symbols: &str) -> Self {
        let mut cloned = self.clone();
        cloned.padding_symbols = symbols.to_string();
        cloned
    }

    fn with_padding_symbol_lengths(&self, prefix: u8, suffix: u8) -> Self {
        let mut cloned = self.clone();
        cloned.padding_symbol_lengths = (prefix, suffix);
        cloned.padding_strategy = PaddingStrategy::Fixed;
        cloned
    }

    fn with_padding_strategy(
        &self,
        padding_strategy: PaddingStrategy,
    ) -> Result<Settings, &'static str> {
        match padding_strategy {
            PaddingStrategy::Adaptive(0) => Err("invalid adaptive padding number"),
            _ => {
                let mut cloned = self.clone();
                cloned.padding_strategy = padding_strategy;
                cloned.padding_symbol_lengths = (0, 0);
                Ok(cloned)
            }
        }
    }

    fn with_word_transforms(&self, transforms: FieldSize) -> Result<Self, &'static str> {
        let mut cloned = self.clone();

        // handle group transforms first
        if transforms.has_flag(WordTransform::AlternatingCaseLowerFirst) {
            cloned.word_transforms = WordTransform::AlternatingCaseLowerFirst as FieldSize;
            return Ok(cloned);
        }

        if transforms.has_flag(WordTransform::AlternatingCaseUpperFirst) {
            cloned.word_transforms = WordTransform::AlternatingCaseUpperFirst as FieldSize;
            return Ok(cloned);
        }

        // no transform matched
        if !transforms.has_flag(WordTransform::Lowercase)
            && !transforms.has_flag(WordTransform::Titlecase)
            && !transforms.has_flag(WordTransform::Uppercase)
            && !transforms.has_flag(WordTransform::InversedTitlecase)
        {
            return Err("invalid transform");
        }

        let mut cloned = self.clone();
        cloned.word_transforms = transforms;
        Ok(cloned)
    }

    fn from_preset(preset: Preset) -> Self {
        match preset {
            Preset::AppleID => Settings {
                words_count: 3,
                word_lengths: (5, 7),
                word_transforms: WordTransform::Lowercase | WordTransform::Uppercase,
                separators: "-:.,".to_string(),
                padding_digits: (2, 2),
                padding_symbols: "!?@&".to_string(),
                padding_symbol_lengths: (1, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Default => Settings {
                words_count: 3,
                word_lengths: (4, 8),
                word_transforms: FieldSize::from_flag(WordTransform::AlternatingCaseLowerFirst),
                separators: DEFAULT_SYMBOLS.to_string(),
                padding_digits: (2, 2),
                padding_symbols: DEFAULT_SYMBOLS.to_string(),
                padding_symbol_lengths: (2, 2),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::WindowsNTLMv1 => Settings {
                words_count: 2,
                word_lengths: (5, 5),
                word_transforms: FieldSize::from_flag(WordTransform::InversedTitlecase),
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (1, 0),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (0, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::SecurityQuestions => Settings {
                words_count: 6,
                word_lengths: (4, 8),
                word_transforms: FieldSize::from_flag(WordTransform::Lowercase),
                separators: " ".to_string(),
                padding_digits: (0, 0),
                padding_symbols: ".!?".to_string(),
                padding_symbol_lengths: (0, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Web16 => Settings {
                words_count: 3,
                word_lengths: (4, 4),
                word_transforms: WordTransform::Lowercase | WordTransform::Uppercase,
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (0, 0),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (1, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Web32 => Settings {
                words_count: 4,
                word_lengths: (4, 5),
                word_transforms: FieldSize::from_flag(WordTransform::AlternatingCaseUpperFirst),
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (2, 2),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (1, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Wifi => Settings {
                words_count: 6,
                word_lengths: (4, 8),
                word_transforms: WordTransform::Lowercase | WordTransform::Uppercase,
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (4, 4),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (0, 0),
                padding_strategy: PaddingStrategy::Adaptive(63),
            },
            Preset::XKCD => Settings {
                words_count: 4,
                word_lengths: (4, 8),
                word_transforms: WordTransform::Lowercase | WordTransform::Uppercase,
                separators: "-".to_string(),
                padding_digits: (0, 0),
                padding_symbols: "".to_string(),
                padding_symbol_lengths: (0, 0),
                padding_strategy: PaddingStrategy::Fixed,
            },
        }
    }
}

impl Randomizer for Settings {
    fn word_lengths(&self) -> Range<u8> {
        let (min, max) = self.word_lengths;
        min..(max + 1)
    }

    fn rand_words(&self, pool: &[&str]) -> Vec<String> {
        let words_list = self.build_words_list(pool);
        let transforms_list = self.build_transforms_list();

        words_list
            .iter()
            .zip(transforms_list.iter())
            .map(|(word, &transform)| transform_word(word, transform))
            .collect()
    }

    fn rand_separator(&self) -> String {
        rand_chars(&self.separators, 1)
    }

    fn rand_prefix(&self) -> (String, String) {
        let (prefix_digits, _) = self.padding_digits;
        let (prefix_symbols, _) = self.padding_symbol_lengths;
        (
            rand_chars(&self.padding_symbols, prefix_symbols),
            rand_digits(prefix_digits),
        )
    }

    fn rand_suffix(&self) -> (String, String) {
        let (_, suffix_digits) = self.padding_digits;
        let (_, suffix_symbols) = self.padding_symbol_lengths;
        (
            rand_digits(suffix_digits),
            rand_chars(&self.padding_symbols, suffix_symbols),
        )
    }

    fn adjust_padding(&self, pass_length: usize) -> PaddingResult {
        match self.padding_strategy {
            PaddingStrategy::Fixed => PaddingResult::Unchanged,
            PaddingStrategy::Adaptive(len) => {
                let length = len as usize;

                if length > pass_length {
                    let padded_symbols =
                        rand_chars(&self.padding_symbols, (length - pass_length) as u8);
                    PaddingResult::Pad(padded_symbols)
                } else {
                    PaddingResult::Trim((pass_length - length) as u8)
                }
            }
        }
    }
}

impl Settings {
    const ALL_SINGLE_WORD_TRANSFORMS: [WordTransform; 4] = [
        WordTransform::Lowercase,
        WordTransform::Titlecase,
        WordTransform::Uppercase,
        WordTransform::InversedTitlecase,
    ];

    fn build_words_list<'a>(&self, pool: &[&'a str]) -> Vec<&'a str> {
        if pool.is_empty() {
            return vec![];
        }

        let mut rng = rand::thread_rng();
        let word_indices = Uniform::from(0..pool.len());

        // not enough words to distinguishably randomize
        if pool.len() < self.words_count as usize {
            return (0..self.words_count)
                .map(|_| {
                    let index: usize = word_indices.sample(&mut rng);
                    pool[index]
                })
                .collect();
        }

        // enough words, ensure no duplicates
        let mut index_marker: HashMap<usize, bool> = HashMap::new();
        (0..self.words_count)
            .map(|_| loop {
                let index: usize = word_indices.sample(&mut rng);
                let word = pool[index];

                if index_marker.get(&index).is_none() {
                    index_marker.insert(index, true);
                    break word;
                }
            })
            .collect()
    }

    fn build_transforms_list(&self) -> Vec<WordTransform> {
        if self
            .word_transforms
            .has_flag(WordTransform::AlternatingCaseLowerFirst)
        {
            return (0..self.words_count)
                .map(|idx| {
                    if idx % 2 == 0 {
                        WordTransform::Lowercase
                    } else {
                        WordTransform::Uppercase
                    }
                })
                .collect();
        }

        if self
            .word_transforms
            .has_flag(WordTransform::AlternatingCaseUpperFirst)
        {
            return (0..self.words_count)
                .map(|idx| {
                    if idx % 2 == 0 {
                        WordTransform::Uppercase
                    } else {
                        WordTransform::Lowercase
                    }
                })
                .collect();
        }

        let whitelisted_transforms: Vec<&WordTransform> = Self::ALL_SINGLE_WORD_TRANSFORMS
            .iter()
            .filter(|&&transform| self.word_transforms & transform)
            .collect();

        let mut rng = rand::thread_rng();
        let transform_indices = Uniform::from(0..whitelisted_transforms.len());

        (0..self.words_count)
            .map(|_| {
                let index: usize = transform_indices.sample(&mut rng);
                *whitelisted_transforms[index]
            })
            .collect()
    }
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

fn transform_word(word: &str, transform: WordTransform) -> String {
    match transform {
        WordTransform::Titlecase => word[..1].to_uppercase() + &word[1..],
        WordTransform::Uppercase => word.to_uppercase(),
        WordTransform::InversedTitlecase => word[..1].to_lowercase() + &word[1..].to_uppercase(),
        // lowercase by default
        _ => word.to_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();

        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));
    }

    #[test]
    fn test_with_words_count() {
        // invalid value
        assert!(matches!(
            Settings::default().with_words_count(0),
            Err("only positive integer is allowed for words count")
        ));

        let settings = Settings::default().with_words_count(1).unwrap();
        // only words_count updated
        assert_eq!(1, settings.words_count);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

        // overriding with multiple calls
        let other_settings = settings.with_words_count(123).unwrap();
        assert_eq!(123, other_settings.words_count);
    }

    #[test]
    fn test_with_word_lengths() {
        // invalid lengths
        assert!(matches!(
            Settings::default().with_word_lengths(MIN_WORD_LENGTH - 1, MAX_WORD_LENGTH + 1),
            Err(MIN_WORD_LENGTH_ERR)
        ));

        // max word length has lower priority
        assert!(matches!(
            Settings::default().with_word_lengths(MIN_WORD_LENGTH, MAX_WORD_LENGTH + 1),
            Err(MAX_WORD_LENGTH_ERR)
        ));

        let settings = Settings::default().with_word_lengths(4, 6).unwrap();
        // only word_lengths updated
        assert_eq!((4, 6), settings.word_lengths);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

        // overriding with multiple calls
        let other_settings = settings.with_word_lengths(5, 5).unwrap();
        assert_eq!((5, 5), other_settings.word_lengths); // equal values

        let other_settings = settings.with_word_lengths(6, 4).unwrap();
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
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

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
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

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
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

        // overriding with multiple calls
        let other_settings = settings.with_padding_symbols("def789");
        assert_eq!("def789", other_settings.padding_symbols);
    }

    #[test]
    fn test_with_padding_symbol_lengths() {
        let settings = Settings::default()
            .with_padding_strategy(PaddingStrategy::Adaptive(12))
            .unwrap()
            .with_padding_symbol_lengths(3, 4);
        // only padding_symbol_lengths and padding_strategy updated
        assert_eq!((3, 4), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);

        // overriding with multiple calls
        let other_settings = settings.with_padding_symbol_lengths(0, 0);
        assert_eq!((0, 0), other_settings.padding_symbol_lengths);
    }

    #[test]
    fn test_with_padding_strategy() {
        // invalid adaptive padding
        assert!(matches!(
            Settings::default().with_padding_strategy(PaddingStrategy::Adaptive(0)),
            Err("invalid adaptive padding number")
        ));

        let settings = Settings::default()
            .with_padding_strategy(PaddingStrategy::Adaptive(16))
            .unwrap();
        // only padding_strategy and padding_symbol_lengths updated
        assert!(matches!(
            settings.padding_strategy,
            PaddingStrategy::Adaptive(16)
        ));
        assert_eq!((0, 0), settings.padding_symbol_lengths);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);

        // overriding
        let other_settings = settings
            .with_padding_strategy(PaddingStrategy::Adaptive(32))
            .unwrap();
        assert!(matches!(
            other_settings.padding_strategy,
            PaddingStrategy::Adaptive(32)
        ));

        let other_settings = settings
            .with_padding_strategy(PaddingStrategy::Fixed)
            .unwrap();
        assert!(matches!(
            other_settings.padding_strategy,
            PaddingStrategy::Fixed
        ));
    }

    #[test]
    fn test_with_word_transforms_single() {
        // invalid transform
        let table = [0b00010000, 0b00100000];

        for transform in table {
            match Settings::default().with_word_transforms(transform) {
                Ok(_) => panic!("unexpected result"),
                Err(msg) => assert_eq!("invalid transform", msg),
            }
        }

        let settings = Settings::default()
            .with_word_transforms(FieldSize::from_flag(WordTransform::Lowercase))
            .unwrap();

        // only words_transform updated
        assert_eq!(
            FieldSize::from_flag(WordTransform::Lowercase),
            settings.word_transforms
        );

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

        for flag in [
            WordTransform::Titlecase,
            WordTransform::Uppercase,
            WordTransform::InversedTitlecase,
        ] {
            let transform = FieldSize::from_flag(flag);
            let other_settings = settings.with_word_transforms(transform).unwrap();
            assert_eq!(transform, other_settings.word_transforms);
        }
    }

    #[test]
    fn test_with_word_transforms_group() {
        for group_flag in [
            WordTransform::AlternatingCaseLowerFirst,
            WordTransform::AlternatingCaseUpperFirst,
        ] {
            for single_flag in [
                WordTransform::Lowercase,
                WordTransform::Titlecase,
                WordTransform::Uppercase,
                WordTransform::InversedTitlecase,
            ] {
                let settings = Settings::default()
                    .with_word_transforms(group_flag | single_flag)
                    .unwrap();
                // only words_transform updated
                assert_eq!(FieldSize::from_flag(group_flag), settings.word_transforms);
            }
        }
    }

    #[test]
    fn test_get_word_lengths() {
        let table = [((4, 6), 4..7), ((5, 5), 5..6), ((6, 10), 6..11)];

        for ((min, max), expected_lengths) in table {
            let settings = Settings::default().with_word_lengths(min, max).unwrap();
            assert_eq!(expected_lengths, settings.word_lengths());
        }
    }

    #[test]
    fn test_rand_words() {
        let settings = Settings::default()
            .with_words_count(3)
            .unwrap()
            .with_word_transforms(FieldSize::from_flag(WordTransform::Uppercase))
            .unwrap();

        // empty pool
        assert!(settings.rand_words(&vec![] as &Vec<&str>).is_empty());

        // not enough pool
        let words = settings.rand_words(&["foo", "bar"]);
        assert_eq!(3, words.len());

        // enough pool
        let words = settings.rand_words(&["foo", "bar", "barz"]);
        assert_eq!(3, words.len());
        assert_eq!(
            HashSet::from([&"FOO".to_string(), &"BAR".to_string(), &"BARZ".to_string()]),
            words.iter().collect::<HashSet<&String>>()
        );
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
            let (symbols, digits) = settings.rand_prefix();
            assert_eq!("", symbols);
            assert_eq!("", digits);
        }

        for prefix_symbols in 1usize..10 {
            for prefix_digits in 1usize..10 {
                let settings = Settings::default()
                    .with_padding_digits(prefix_digits as u8, 2)
                    .with_padding_symbols("#")
                    .with_padding_symbol_lengths(prefix_symbols as u8, 3);
                let (symbols, digits) = settings.rand_prefix();

                // total length of prefix
                assert_eq!(prefix_symbols, symbols.len());
                assert_eq!(prefix_digits, digits.len());

                // first part is the repeated symbol
                assert_eq!("#".to_string().repeat(prefix_symbols), symbols);

                // second part is the stringified digits
                let _ = digits.parse::<u64>().unwrap();
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
            let (digits, symbols) = settings.rand_suffix();
            assert_eq!("", digits);
            assert_eq!("", symbols);
        }

        for suffix_symbols in 1usize..10 {
            for suffix_digits in 1usize..10 {
                let settings = Settings::default()
                    .with_padding_digits(2, suffix_digits as u8)
                    .with_padding_symbols("~")
                    .with_padding_symbol_lengths(3, suffix_symbols as u8);
                let (digits, symbols) = settings.rand_suffix();

                // total length of suffix
                assert_eq!(suffix_digits, digits.len());
                assert_eq!(suffix_symbols, symbols.len());

                // first part is the stringified digits
                let _ = digits.parse::<u64>().unwrap();

                // second part is repeated symbols
                assert_eq!("~".to_string().repeat(suffix_symbols), symbols);
            }
        }
    }

    #[test]
    fn test_adjust_padding() {
        let pass_length = 12;

        // fixed padding
        let settings = Settings::default()
            .with_padding_strategy(PaddingStrategy::Fixed)
            .unwrap();
        assert!(matches!(
            settings.adjust_padding(pass_length),
            PaddingResult::Unchanged
        ));

        // adaptive padding: add symbols
        let settings = Settings::default()
            .with_padding_symbols("@")
            .with_padding_strategy(PaddingStrategy::Adaptive(15))
            .unwrap();
        match settings.adjust_padding(pass_length) {
            PaddingResult::Pad(padded_symbols) => assert_eq!("@@@", padded_symbols),
            _ => panic!("invalid padding result"),
        }

        // adaptive padding: cut length
        let settings = Settings::default()
            .with_padding_strategy(PaddingStrategy::Adaptive(10))
            .unwrap();
        assert!(matches!(
            settings.adjust_padding(pass_length),
            PaddingResult::Trim(2)
        ));
    }

    #[test]
    fn test_build_words_list() {
        let settings = Settings::default().with_words_count(3).unwrap();

        // empty pool
        assert!(settings.build_words_list(&vec![] as &Vec<&str>).is_empty());

        // pool size smaller than words count
        let pool = &["foo", "bar"];

        for _ in 0..10 {
            let words = settings.build_words_list(pool);
            assert_eq!(3, words.len());

            let unique_words: HashSet<String> =
                words.iter().map(|word| word.to_lowercase()).collect();
            assert!(unique_words.len() < 3);
        }

        // enough pool
        let pool = &["foo", "bar", "fooz", "barz"];

        for _ in 0..10 {
            let words = settings.build_words_list(pool);
            assert_eq!(3, words.len());

            let unique_words: HashSet<String> =
                words.iter().map(|word| word.to_lowercase()).collect();
            assert_eq!(3, unique_words.len());
        }
    }

    #[test]
    fn test_build_transforms_list() {
        let all_transforms = WordTransform::Lowercase
            | WordTransform::Titlecase
            | WordTransform::Uppercase
            | WordTransform::InversedTitlecase;

        let settings = Settings::default()
            .with_words_count(3)
            .unwrap()
            .with_word_transforms(all_transforms)
            .unwrap();

        let transforms_list = settings.build_transforms_list();
        assert_eq!(3, transforms_list.len());

        let table = [
            (
                FieldSize::from_flag(WordTransform::AlternatingCaseLowerFirst),
                vec![
                    WordTransform::Lowercase,
                    WordTransform::Uppercase,
                    WordTransform::Lowercase,
                ],
            ),
            (
                FieldSize::from_flag(WordTransform::AlternatingCaseUpperFirst),
                vec![
                    WordTransform::Uppercase,
                    WordTransform::Lowercase,
                    WordTransform::Uppercase,
                ],
            ),
        ];

        for (group_transform, expected) in table {
            let settings = Settings::default()
                .with_words_count(3)
                .unwrap()
                .with_word_transforms(all_transforms | group_transform)
                .unwrap();
            let transforms_list = settings.build_transforms_list();
            assert_eq!(expected, transforms_list);
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
                let result = rand_chars(DEFAULT_SYMBOLS, count);
                assert_eq!(count as usize, result.len());
                assert_eq!(
                    result
                        .chars()
                        .nth(0)
                        .unwrap()
                        .to_string()
                        .repeat(count as usize),
                    result
                );
            }
        }
    }

    #[test]
    fn test_transform_word() {
        let table = [
            (
                WordTransform::Lowercase,
                [
                    ("foo", "foo"),
                    ("Bar", "bar"),
                    ("1Fooz", "1fooz"),
                    ("123", "123"),
                ],
            ),
            (
                WordTransform::Titlecase,
                [
                    ("foo", "Foo"),
                    ("Bar", "Bar"),
                    ("1Fooz", "1Fooz"),
                    ("123", "123"),
                ],
            ),
            (
                WordTransform::Uppercase,
                [
                    ("foo", "FOO"),
                    ("Bar", "BAR"),
                    ("1Fooz", "1FOOZ"),
                    ("123", "123"),
                ],
            ),
            (
                WordTransform::InversedTitlecase,
                [
                    ("foo", "fOO"),
                    ("Bar", "bAR"),
                    ("1Fooz", "1FOOZ"),
                    ("123", "123"),
                ],
            ),
        ];

        for (transform, cases) in table {
            for (word, expected) in cases {
                assert_eq!(expected, transform_word(word, transform));
            }
        }
    }
}
