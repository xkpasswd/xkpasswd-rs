use super::*;
use proptest::prelude::*;
use std::collections::HashMap;

// Basic property-based tests for the prelude module

proptest! {
    #[test]
    fn test_guess_time_entropy_consistency(entropy in 0usize..100) {
        let time1 = GuessTime::for_entropy(entropy);
        let time2 = GuessTime::for_entropy(entropy);
        prop_assert_eq!(time1, time2);
    }

    #[test]
    fn test_higher_entropy_longer_time(e1 in 0usize..50, e2 in 51usize..100) {
        let time1 = GuessTime::for_entropy(e1);
        let time2 = GuessTime::for_entropy(e2);

        // Convert to total days for comparison
        let days1 = time1.years * 365 + time1.months as usize * 30 + time1.days as usize;
        let days2 = time2.years * 365 + time2.months as usize * 30 + time2.days as usize;

        prop_assert!(days2 >= days1);
    }
}

// Test entropy thresholds
#[test]
fn test_entropy_thresholds() {
    // Test entropy threshold 44 (45-54 range)
    for entropy in 45..=54 {
        let time = GuessTime::for_entropy(entropy);
        assert_eq!(time.years, 1001);
        assert_eq!(time.months, 0);
        assert_eq!(time.days, 0);
    }

    // Test entropy threshold 54 (55-64 range)
    for entropy in 55..=64 {
        let time = GuessTime::for_entropy(entropy);
        assert_eq!(time.years, 1_000_001);
        assert_eq!(time.months, 0);
        assert_eq!(time.days, 0);
    }

    // Test entropy threshold 64 (above 64)
    for entropy in 65..75 {
        let time = GuessTime::for_entropy(entropy);
        assert_eq!(time.years, 1_000_000_001);
        assert_eq!(time.months, 0);
        assert_eq!(time.days, 0);
    }
}

proptest! {
    #[test]
    fn test_guess_time_display_zero_values(years in 0usize..10, months in 0u8..12, days in 0u8..30) {
        let time = GuessTime { years, months, days };
        let display = format!("{}", time);

        prop_assert!(!display.is_empty());

        if years == 0 && months == 0 && days == 0 {
            prop_assert_eq!(display, "less than a day");
        }
    }
}

// Test display formatting
#[test]
fn test_guess_time_display_large_values() {
    let time = GuessTime {
        years: 1_000_000_001,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "more than a billion years");

    let time = GuessTime {
        years: 1_000_001,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "more than a million years");

    let time = GuessTime {
        years: 1_001,
        months: 0,
        days: 0,
    };
    assert_eq!(format!("{}", time), "more than a thousand years");
}

proptest! {
    #[test]
    fn test_entropy_display_equal_min_max(blind_entropy in 10usize..100, seen_entropy in 10usize..100) {
        let entropy = Entropy {
            blind_min: blind_entropy,
            blind_max: blind_entropy,
            seen: seen_entropy,
            guess_time: GuessTime::for_entropy(seen_entropy),
        };

        let display = format!("{}", entropy);
        let expected_blind = format!("{} bits blind", blind_entropy);
        prop_assert!(display.contains(&expected_blind));
        prop_assert!(!display.contains("between"));
    }

    #[test]
    fn test_entropy_display_different_min_max(
        blind_min in 10usize..50,
        blind_max in 51usize..100,
        seen_entropy in 10usize..100
    ) {
        let entropy = Entropy {
            blind_min,
            blind_max,
            seen: seen_entropy,
            guess_time: GuessTime::for_entropy(seen_entropy),
        };

        let display = format!("{}", entropy);
        let expected_between = format!("between {} & {} bits", blind_min, blind_max);
        let expected_knowledge = format!("{} bits with full knowledge", seen_entropy);
        prop_assert!(display.contains(&expected_between));
        prop_assert!(display.contains(&expected_knowledge));
    }
}

proptest! {
    #[test]
    fn test_dict_loading_valid_format(word_count in 1usize..10, word_len in 3u8..8) {
        let words: Vec<String> = (0..word_count)
            .map(|i| format!("word{}", i))
            .collect();
        let dict_line = format!("{}:{}", word_len, words.join(","));
        let dict_bytes = dict_line.as_bytes();

        let dict = load_dict(dict_bytes);

        prop_assert_eq!(dict.len(), 1);
        prop_assert!(dict.contains_key(&word_len));

        let loaded_words = dict.get(&word_len).unwrap();
        prop_assert_eq!(loaded_words.len(), word_count);

        for (i, word) in loaded_words.iter().enumerate() {
            prop_assert_eq!(*word, format!("word{}", i));
        }
    }
}

// Test dict loading with static tests to avoid lifetime issues
#[test]
fn test_dict_loading_multiple_lengths() {
    let dict_content = "3:foo,bar,baz\n5:hello,world";
    let dict_bytes = dict_content.as_bytes();

    let dict = load_dict(dict_bytes);

    assert_eq!(dict.len(), 2);
    assert!(dict.contains_key(&3));
    assert!(dict.contains_key(&5));

    let words3 = dict.get(&3).unwrap();
    let words5 = dict.get(&5).unwrap();

    assert_eq!(words3.len(), 3);
    assert_eq!(words5.len(), 2);

    assert_eq!(words3, &vec!["foo", "bar", "baz"]);
    assert_eq!(words5, &vec!["hello", "world"]);
}

#[test]
fn test_dict_loading_empty_input() {
    let dict = load_dict(&[]);
    assert!(dict.is_empty());

    let dict = load_dict("".as_bytes());
    assert!(dict.is_empty());

    let dict = load_dict("   \n  \n  ".as_bytes());
    assert!(dict.is_empty());
}

// Mock settings for testing password generation
#[derive(Clone)]
struct PropTestMockSettings {
    words_count: u8,
    word_lengths: std::ops::Range<u8>,
    separator: String,
    prefix_digits: usize,
    suffix_digits: usize,
    prefix_symbols: usize,
    suffix_symbols: usize,
    padding_result: PaddingResult,
    words: Vec<String>,
}

impl Randomizer for PropTestMockSettings {
    fn word_lengths(&self) -> std::ops::Range<u8> {
        self.word_lengths.clone()
    }

    fn rand_words(&self, _pool: &[&str]) -> Vec<String> {
        self.words
            .iter()
            .take(self.words_count as usize)
            .cloned()
            .collect()
    }

    fn rand_separator(&self) -> String {
        self.separator.clone()
    }

    fn rand_prefix(&self) -> (String, String) {
        let prefix_symbols = "?".repeat(self.prefix_symbols);
        let prefix_digits = "1".repeat(self.prefix_digits);
        (prefix_symbols, prefix_digits)
    }

    fn rand_suffix(&self) -> (String, String) {
        let suffix_digits = "9".repeat(self.suffix_digits);
        let suffix_symbols = "!".repeat(self.suffix_symbols);
        (suffix_digits, suffix_symbols)
    }

    fn adjust_padding(&self, _pass_length: usize) -> PaddingResult {
        self.padding_result.clone()
    }

    fn calc_entropy(&self, pool_size: usize) -> Entropy {
        let entropy_bits = (pool_size as f64).log2() * self.words_count as f64;
        Entropy {
            blind_min: entropy_bits as usize,
            blind_max: entropy_bits as usize,
            seen: entropy_bits as usize,
            guess_time: GuessTime::for_entropy(entropy_bits as usize),
        }
    }
}

// Password generation tests
#[test]
fn test_password_generation_basic() {
    let settings = PropTestMockSettings {
        words_count: 3,
        word_lengths: 3..8,
        separator: ".".to_string(),
        prefix_digits: 0,
        suffix_digits: 0,
        prefix_symbols: 0,
        suffix_symbols: 0,
        padding_result: PaddingResult::Unchanged,
        words: vec!["test".to_string(), "word".to_string(), "pass".to_string()],
    };

    let mut dict: HashMap<u8, Vec<&str>> = HashMap::new();
    dict.insert(4, vec!["test", "word", "pass"]);

    let xkpasswd = Xkpasswd { dict };
    let (password, _entropy) = xkpasswd.gen_pass(&settings);

    assert_eq!(password, "test.word.pass");
}

#[test]
fn test_password_generation_with_padding() {
    let settings = PropTestMockSettings {
        words_count: 2,
        word_lengths: 3..8,
        separator: "-".to_string(),
        prefix_digits: 1,
        suffix_digits: 1,
        prefix_symbols: 1,
        suffix_symbols: 1,
        padding_result: PaddingResult::Unchanged,
        words: vec!["foo".to_string(), "bar".to_string()],
    };

    let mut dict: HashMap<u8, Vec<&str>> = HashMap::new();
    dict.insert(3, vec!["foo", "bar"]);

    let xkpasswd = Xkpasswd { dict };
    let (password, _entropy) = xkpasswd.gen_pass(&settings);

    assert_eq!(password, "?1-foo-bar-9!");
}

#[test]
fn test_password_generation_trim_padding() {
    let settings = PropTestMockSettings {
        words_count: 3,
        word_lengths: 3..8,
        separator: ".".to_string(),
        prefix_digits: 0,
        suffix_digits: 0,
        prefix_symbols: 0,
        suffix_symbols: 0,
        padding_result: PaddingResult::TrimTo(8),
        words: vec!["test".to_string(), "word".to_string(), "pass".to_string()],
    };

    let mut dict: HashMap<u8, Vec<&str>> = HashMap::new();
    dict.insert(4, vec!["test", "word", "pass"]);

    let xkpasswd = Xkpasswd { dict };
    let (password, _entropy) = xkpasswd.gen_pass(&settings);

    assert_eq!(password, "test.wor");
    assert_eq!(password.len(), 8);
}

#[test]
fn test_password_generation_pad_padding() {
    let settings = PropTestMockSettings {
        words_count: 2,
        word_lengths: 3..8,
        separator: ".".to_string(),
        prefix_digits: 0,
        suffix_digits: 0,
        prefix_symbols: 0,
        suffix_symbols: 0,
        padding_result: PaddingResult::Pad("###".to_string()),
        words: vec!["foo".to_string(), "bar".to_string()],
    };

    let mut dict: HashMap<u8, Vec<&str>> = HashMap::new();
    dict.insert(3, vec!["foo", "bar"]);

    let xkpasswd = Xkpasswd { dict };
    let (password, _entropy) = xkpasswd.gen_pass(&settings);

    assert_eq!(password, "foo.bar###");
}
