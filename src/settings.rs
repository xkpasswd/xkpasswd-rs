use super::prelude::{Builder, Randomizer};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::cmp;
use std::collections::HashMap;
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
const DEFAULT_WORD_TRANSFORMS: u8 = WordTransform::LOWERCASE | WordTransform::UPPERCASE;

#[derive(Clone, Debug)]
pub enum PaddingStrategy {
    Fixed,
    Adaptive(u8),
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

#[wasm_bindgen(js_name = "WordTransform")]
pub enum WasmWordTransform {
    // single transforms - possible to combine with each other
    Lowercase = 0b00000001,
    Titlecase = 0b00000010,
    Uppercase = 0b00000100,
    InversedTitlecase = 0b00001000,

    // group transforms - overriding other single ones
    AlternatingCaseLowerFirst = 0b01000000,
    AlternatingCaseUpperFirst = 0b10000000,
}

pub struct WordTransform;

impl WordTransform {
    // single transforms - possible to combine with each other
    pub const LOWERCASE: u8 = 1;
    pub const TITLECASE: u8 = 1 << 1;
    pub const UPPERCASE: u8 = 1 << 2;
    pub const INVERSED_TITLECASE: u8 = 1 << 3;

    // group transforms - overriding other single ones
    pub const ALTERNATING_CASE_LOWER_FIRST: u8 = 1 << 6;
    pub const ALTERNATING_CASE_UPPER_FIRST: u8 = 1 << 7;
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
                Ok(cloned)
            }
        }
    }

    fn with_word_transforms(&self, transforms: u8) -> Result<Self, &'static str> {
        let mut cloned = self.clone();

        // handle group transforms first
        if transforms & WordTransform::ALTERNATING_CASE_LOWER_FIRST > 0 {
            cloned.word_transforms = WordTransform::ALTERNATING_CASE_LOWER_FIRST;
            return Ok(cloned);
        }

        if transforms & WordTransform::ALTERNATING_CASE_UPPER_FIRST > 0 {
            cloned.word_transforms = WordTransform::ALTERNATING_CASE_UPPER_FIRST;
            return Ok(cloned);
        }

        // no transform matched
        if transforms & WordTransform::LOWERCASE == 0
            && transforms & WordTransform::TITLECASE == 0
            && transforms & WordTransform::UPPERCASE == 0
            && transforms & WordTransform::INVERSED_TITLECASE == 0
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
                word_transforms: WordTransform::LOWERCASE | WordTransform::UPPERCASE,
                separators: "-:.,".to_string(),
                padding_digits: (2, 2),
                padding_symbols: "!?@&".to_string(),
                padding_symbol_lengths: (1, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Default => Settings {
                words_count: 3,
                word_lengths: (4, 8),
                word_transforms: WordTransform::ALTERNATING_CASE_LOWER_FIRST,
                separators: DEFAULT_SYMBOLS.to_string(),
                padding_digits: (2, 2),
                padding_symbols: DEFAULT_SYMBOLS.to_string(),
                padding_symbol_lengths: (2, 2),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::WindowsNTLMv1 => Settings {
                words_count: 2,
                word_lengths: (5, 5),
                word_transforms: WordTransform::INVERSED_TITLECASE,
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (1, 0),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (0, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::SecurityQuestions => Settings {
                words_count: 6,
                word_lengths: (4, 8),
                word_transforms: WordTransform::LOWERCASE,
                separators: " ".to_string(),
                padding_digits: (0, 0),
                padding_symbols: ".!?".to_string(),
                padding_symbol_lengths: (0, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Web16 => Settings {
                words_count: 3,
                word_lengths: (4, 4),
                word_transforms: WordTransform::LOWERCASE | WordTransform::UPPERCASE,
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (0, 0),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (1, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Web32 => Settings {
                words_count: 4,
                word_lengths: (4, 5),
                word_transforms: WordTransform::ALTERNATING_CASE_UPPER_FIRST,
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (2, 2),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (1, 1),
                padding_strategy: PaddingStrategy::Fixed,
            },
            Preset::Wifi => Settings {
                words_count: 6,
                word_lengths: (4, 8),
                word_transforms: WordTransform::LOWERCASE | WordTransform::UPPERCASE,
                separators: "-+=.*_|~,".to_string(),
                padding_digits: (4, 4),
                padding_symbols: "!@$%^&*+=:|~?".to_string(),
                padding_symbol_lengths: (0, 0),
                padding_strategy: PaddingStrategy::Adaptive(63),
            },
            Preset::XKCD => Settings {
                words_count: 4,
                word_lengths: (4, 8),
                word_transforms: WordTransform::LOWERCASE | WordTransform::UPPERCASE,
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
    fn rand_words(&self, pool: &[&str]) -> Vec<String> {
        let words_list = self.build_words_list(pool);
        let transforms_list = self.build_transforms_list();

        words_list
            .iter()
            .zip(transforms_list.iter())
            .map(|(word, transform)| transform_word(word, *transform))
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

    fn iter_word_lengths<F: FnMut(u8)>(&self, callback: F) {
        let (min, max) = self.word_lengths;
        (min..(max + 1)).for_each(callback);
    }

    fn adjust_for_padding_strategy(&self, passwd: &str) -> String {
        match self.padding_strategy {
            PaddingStrategy::Fixed => passwd.to_string(),
            PaddingStrategy::Adaptive(len) => {
                let length = len as usize;

                if length > passwd.len() {
                    let padded_symbols =
                        rand_chars(&self.padding_symbols, (length - passwd.len()) as u8);
                    passwd.to_string() + &padded_symbols
                } else {
                    passwd[..length].to_string()
                }
            }
        }
    }
}

impl Settings {
    const ALL_SINGLE_WORD_TRANSFORMS: [u8; 4] = [
        WordTransform::LOWERCASE,
        WordTransform::TITLECASE,
        WordTransform::UPPERCASE,
        WordTransform::INVERSED_TITLECASE,
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

    fn build_transforms_list(&self) -> Vec<u8> {
        if self.word_transforms & WordTransform::ALTERNATING_CASE_LOWER_FIRST > 0 {
            return (0..self.words_count)
                .map(|idx| {
                    if idx % 2 == 0 {
                        WordTransform::LOWERCASE
                    } else {
                        WordTransform::UPPERCASE
                    }
                })
                .collect();
        }

        if self.word_transforms & WordTransform::ALTERNATING_CASE_UPPER_FIRST > 0 {
            return (0..self.words_count)
                .map(|idx| {
                    if idx % 2 == 0 {
                        WordTransform::UPPERCASE
                    } else {
                        WordTransform::LOWERCASE
                    }
                })
                .collect();
        }

        let whitelisted_transforms: Vec<&u8> = Self::ALL_SINGLE_WORD_TRANSFORMS
            .iter()
            .filter(|transform| self.word_transforms & *transform != 0)
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

fn transform_word(word: &str, transform: u8) -> String {
    match transform {
        WordTransform::TITLECASE => word[..1].to_uppercase() + &word[1..],
        WordTransform::UPPERCASE => word.to_uppercase(),
        WordTransform::INVERSED_TITLECASE => word[..1].to_lowercase() + &word[1..].to_uppercase(),
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
        let other_settings = settings.with_padding_digits(0, 0);
        assert_eq!((0, 0), other_settings.padding_digits);
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
        // only padding_symbols updated
        assert!(matches!(
            settings.padding_strategy,
            PaddingStrategy::Adaptive(16)
        ));

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);

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
            .with_word_transforms(WordTransform::LOWERCASE)
            .unwrap();
        // only words_transform updated
        assert_eq!(WordTransform::LOWERCASE, settings.word_transforms);

        // other fields remain unchanged
        assert_eq!(DEFAULT_WORDS_COUNT, settings.words_count);
        assert_eq!(DEFAULT_WORD_LENGTHS, settings.word_lengths);
        assert_eq!(DEFAULT_SEPARATORS.to_string(), settings.separators);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_digits);
        assert_eq!(DEFAULT_SYMBOLS.to_string(), settings.padding_symbols);
        assert_eq!((0, DEFAULT_PADDING_LENGTH), settings.padding_symbol_lengths);
        assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

        for transform in [
            WordTransform::TITLECASE,
            WordTransform::UPPERCASE,
            WordTransform::INVERSED_TITLECASE,
        ] {
            let other_settings = settings.with_word_transforms(transform).unwrap();
            assert_eq!(transform, other_settings.word_transforms);
        }
    }

    #[test]
    fn test_with_word_transforms_group() {
        for group_transform in [
            WordTransform::ALTERNATING_CASE_LOWER_FIRST,
            WordTransform::ALTERNATING_CASE_UPPER_FIRST,
        ] {
            for single_transform in [
                WordTransform::LOWERCASE,
                WordTransform::TITLECASE,
                WordTransform::UPPERCASE,
                WordTransform::INVERSED_TITLECASE,
            ] {
                let settings = Settings::default()
                    .with_word_transforms(group_transform | single_transform)
                    .unwrap();
                // only words_transform updated
                assert_eq!(group_transform, settings.word_transforms);
            }
        }
    }

    #[test]
    fn test_rand_words() {
        let settings = Settings::default()
            .with_words_count(3)
            .unwrap()
            .with_word_transforms(WordTransform::UPPERCASE)
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
    fn test_iter_word_lengths() {
        let table = [
            ((4, 6), vec![4, 5, 6]),
            ((5, 5), vec![5]),
            ((6, 10), vec![6, 7, 8, 9, 10]),
        ];

        for ((min, max), expected_lengths) in table {
            let settings = Settings::default().with_word_lengths(min, max).unwrap();
            let mut lengths: Vec<u8> = vec![];
            settings.iter_word_lengths(|len| lengths.push(len));
            assert_eq!(expected_lengths, lengths);
        }
    }

    #[test]
    fn test_adjust_for_padding_strategy() {
        let passwd = "foo.bar.68!!".to_string();

        // fixed padding
        let settings = Settings::default()
            .with_padding_strategy(PaddingStrategy::Fixed)
            .unwrap();
        assert_eq!(passwd, settings.adjust_for_padding_strategy(&passwd));

        // adaptive padding: add symbols
        let settings = Settings::default()
            .with_padding_symbols("@")
            .with_padding_strategy(PaddingStrategy::Adaptive(15))
            .unwrap();
        assert_eq!(
            format!("{}@@@", passwd),
            settings.adjust_for_padding_strategy(&passwd)
        );

        // adaptive padding: cut length
        let settings = Settings::default()
            .with_padding_symbols("@")
            .with_padding_strategy(PaddingStrategy::Adaptive(10))
            .unwrap();
        assert_eq!("foo.bar.68", settings.adjust_for_padding_strategy(&passwd));
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
        let all_transforms = WordTransform::LOWERCASE
            | WordTransform::TITLECASE
            | WordTransform::UPPERCASE
            | WordTransform::INVERSED_TITLECASE;

        let settings = Settings::default()
            .with_words_count(3)
            .unwrap()
            .with_word_transforms(all_transforms)
            .unwrap();

        let transforms_list = settings.build_transforms_list();
        assert_eq!(3, transforms_list.len());

        let table = [
            (
                WordTransform::ALTERNATING_CASE_LOWER_FIRST,
                vec![
                    WordTransform::LOWERCASE,
                    WordTransform::UPPERCASE,
                    WordTransform::LOWERCASE,
                ],
            ),
            (
                WordTransform::ALTERNATING_CASE_UPPER_FIRST,
                vec![
                    WordTransform::UPPERCASE,
                    WordTransform::LOWERCASE,
                    WordTransform::UPPERCASE,
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
                WordTransform::LOWERCASE,
                [
                    ("foo", "foo"),
                    ("Bar", "bar"),
                    ("1Fooz", "1fooz"),
                    ("123", "123"),
                ],
            ),
            (
                WordTransform::TITLECASE,
                [
                    ("foo", "Foo"),
                    ("Bar", "Bar"),
                    ("1Fooz", "1Fooz"),
                    ("123", "123"),
                ],
            ),
            (
                WordTransform::UPPERCASE,
                [
                    ("foo", "FOO"),
                    ("Bar", "BAR"),
                    ("1Fooz", "1FOOZ"),
                    ("123", "123"),
                ],
            ),
            (
                WordTransform::INVERSED_TITLECASE,
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
