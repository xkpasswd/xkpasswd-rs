use super::*;

struct MockSettings {
    padding_digits: (usize, usize),
    padding_symbols: (usize, usize),
    padding_result: PaddingResult,
}

impl Randomizer for MockSettings {
    fn word_lengths(&self) -> Range<u8> {
        3..4
    }

    fn rand_words(&self, _: &[&str]) -> Vec<String> {
        vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
    }

    fn rand_separator(&self) -> String {
        ".".to_string()
    }

    fn rand_prefix(&self) -> (String, String) {
        let prefix_symbols = &"?????"[..self.padding_symbols.0];
        let prefix_digits = &"12345"[..self.padding_digits.0];
        (prefix_symbols.to_string(), prefix_digits.to_string())
    }

    fn rand_suffix(&self) -> (String, String) {
        let suffix_symbols = &"!!!!!!"[..self.padding_symbols.1];
        let suffix_digits = &"67890"[..self.padding_digits.1];
        (suffix_digits.to_string(), suffix_symbols.to_string())
    }

    fn adjust_padding(&self, _: usize) -> PaddingResult {
        match &self.padding_result {
            PaddingResult::Unchanged => PaddingResult::Unchanged,
            PaddingResult::TrimTo(len) => PaddingResult::TrimTo(*len),
            PaddingResult::Pad(str) => PaddingResult::Pad(str.clone()),
        }
    }

    fn calc_entropy(&self, _: usize) -> Entropy {
        Entropy::default()
    }
}

#[test]
fn test_load_dict_blank() {
    let dict = load_dict(&[]);
    assert!(dict.is_empty());

    let dict_bytes = "".as_bytes();
    let dict = load_dict(dict_bytes);
    assert!(dict.is_empty());
}

#[test]
fn test_load_dict_valid_data() {
    let table = [
        "2:an,do\n3:foo,bar",
        r#"
        2:an,do
        3:foo,bar
        "#,
    ];

    for dict_str in table {
        let dict_bytes = dict_str.as_bytes();
        let dict = load_dict(dict_bytes);

        assert_eq!(2, dict.len());
        assert_eq!(vec!["an", "do"], *dict.get(&2).unwrap());
        assert_eq!(vec!["foo", "bar"], *dict.get(&3).unwrap());
        assert!(dict.get(&4).is_none());
    }
}

#[test]
fn test_load_dict_invalid_data() {
    // Invalid format (non-numeric length) should be gracefully skipped
    let dict_bytes = "foo:bar,baz".as_bytes();
    let dict = load_dict(dict_bytes);
    assert!(dict.is_empty());
}

#[cfg(feature = "lang_en")]
#[test]
fn test_xkpasswd_for_en() {
    let pass = Xkpasswd::for_language(Language::English);
    assert!(!pass.dict.is_empty());

    assert!(pass.dict.get(&2).is_none());
    assert!(pass.dict.get(&3).is_none());

    assert_eq!(1500, pass.dict.get(&4).unwrap().len());
    assert_eq!(1500, pass.dict.get(&5).unwrap().len());
    assert_eq!(1500, pass.dict.get(&6).unwrap().len());
    assert_eq!(1500, pass.dict.get(&7).unwrap().len());
    assert_eq!(1500, pass.dict.get(&8).unwrap().len());
    assert_eq!(1338, pass.dict.get(&9).unwrap().len());
    assert_eq!(807, pass.dict.get(&10).unwrap().len());

    assert!(pass.dict.get(&11).is_none());
}

#[cfg(feature = "lang_de")]
#[test]
fn test_xkpasswd_for_de() {
    let pass = Xkpasswd::for_language(Language::German);
    assert!(!pass.dict.is_empty());

    assert!(pass.dict.get(&2).is_none());
    assert!(pass.dict.get(&3).is_none());

    assert_eq!(1277, pass.dict.get(&4).unwrap().len());
    assert_eq!(1500, pass.dict.get(&5).unwrap().len());
    assert_eq!(1500, pass.dict.get(&6).unwrap().len());
    assert_eq!(1500, pass.dict.get(&7).unwrap().len());
    assert_eq!(1500, pass.dict.get(&8).unwrap().len());
    assert_eq!(1500, pass.dict.get(&9).unwrap().len());
    assert_eq!(1185, pass.dict.get(&10).unwrap().len());

    assert!(pass.dict.get(&11).is_none());
}

#[cfg(feature = "lang_es")]
#[test]
fn test_xkpasswd_for_es() {
    let pass = Xkpasswd::for_language(Language::Spanish);
    assert!(!pass.dict.is_empty());

    assert!(pass.dict.get(&2).is_none());
    assert!(pass.dict.get(&3).is_none());

    assert_eq!(1111, pass.dict.get(&4).unwrap().len());
    assert_eq!(1500, pass.dict.get(&5).unwrap().len());
    assert_eq!(1500, pass.dict.get(&6).unwrap().len());
    assert_eq!(1500, pass.dict.get(&7).unwrap().len());
    assert_eq!(1500, pass.dict.get(&8).unwrap().len());
    assert_eq!(1500, pass.dict.get(&9).unwrap().len());
    assert_eq!(1129, pass.dict.get(&10).unwrap().len());

    assert!(pass.dict.get(&11).is_none());
}

#[cfg(feature = "lang_fr")]
#[test]
fn test_xkpasswd_for_fr() {
    let pass = Xkpasswd::for_language(Language::French);
    assert!(!pass.dict.is_empty());

    assert!(pass.dict.get(&2).is_none());
    assert!(pass.dict.get(&3).is_none());

    assert_eq!(1212, pass.dict.get(&4).unwrap().len());
    assert_eq!(1500, pass.dict.get(&5).unwrap().len());
    assert_eq!(1500, pass.dict.get(&6).unwrap().len());
    assert_eq!(1500, pass.dict.get(&7).unwrap().len());
    assert_eq!(1500, pass.dict.get(&8).unwrap().len());
    assert_eq!(1438, pass.dict.get(&9).unwrap().len());
    assert_eq!(902, pass.dict.get(&10).unwrap().len());

    assert!(pass.dict.get(&11).is_none());
}

#[cfg(feature = "lang_pt")]
#[test]
fn test_xkpasswd_for_pt() {
    let pass = Xkpasswd::for_language(Language::Portuguese);
    assert!(!pass.dict.is_empty());

    assert!(pass.dict.get(&2).is_none());
    assert!(pass.dict.get(&3).is_none());

    assert_eq!(1130, pass.dict.get(&4).unwrap().len());
    assert_eq!(1500, pass.dict.get(&5).unwrap().len());
    assert_eq!(1500, pass.dict.get(&6).unwrap().len());
    assert_eq!(1500, pass.dict.get(&7).unwrap().len());
    assert_eq!(1500, pass.dict.get(&8).unwrap().len());
    assert_eq!(1397, pass.dict.get(&9).unwrap().len());
    assert_eq!(925, pass.dict.get(&10).unwrap().len());

    assert!(pass.dict.get(&11).is_none());
}

#[test]
fn test_xkpasswd_gen_pass() {
    let pass = Xkpasswd::default();
    let table = [
        (
            "foo.bar.baz",
            MockSettings {
                padding_digits: (0, 0),
                padding_symbols: (0, 0),
                padding_result: PaddingResult::Unchanged,
            },
        ),
        (
            "foo.bar.baz.67!!",
            MockSettings {
                padding_digits: (0, 2),
                padding_symbols: (0, 2),
                padding_result: PaddingResult::Unchanged,
            },
        ),
        (
            "??12.foo.bar.baz",
            MockSettings {
                padding_digits: (2, 0),
                padding_symbols: (2, 0),
                padding_result: PaddingResult::Unchanged,
            },
        ),
        (
            "?1.foo.bar.baz.67!!",
            MockSettings {
                padding_digits: (1, 2),
                padding_symbols: (1, 2),
                padding_result: PaddingResult::Unchanged,
            },
        ),
        (
            "??foo.bar.baz.67!!",
            MockSettings {
                padding_digits: (0, 2),
                padding_symbols: (2, 2),
                padding_result: PaddingResult::Unchanged,
            },
        ),
        (
            "?1.foo.bar.baz!!",
            MockSettings {
                padding_digits: (1, 0),
                padding_symbols: (1, 2),
                padding_result: PaddingResult::Unchanged,
            },
        ),
        (
            "?????12345.foo.bar.baz",
            MockSettings {
                padding_digits: (5, 5),
                padding_symbols: (5, 5),
                padding_result: PaddingResult::TrimTo(22),
            },
        ),
        (
            "??12.foo.bar.baz$$$$$",
            MockSettings {
                padding_digits: (2, 0),
                padding_symbols: (2, 0),
                padding_result: PaddingResult::Pad("$$$$$".to_string()),
            },
        ),
    ];

    for (expected, settings) in table {
        let (passwd, _) = pass.gen_pass(&settings);
        assert_eq!(expected, passwd);
    }
}

#[test]
fn test_guess_time_display() {
    // Test "more than a billion years"
    let time = GuessTime {
        years: 2_000_000_000,
        months: 0,
        days: 0,
    };
    assert_eq!("more than a billion years", time.to_string());

    // Test "more than a million years"
    let time = GuessTime {
        years: 5_000_000,
        months: 0,
        days: 0,
    };
    assert_eq!("more than a million years", time.to_string());

    // Test "more than a thousand years"
    let time = GuessTime {
        years: 5_000,
        months: 0,
        days: 0,
    };
    assert_eq!("more than a thousand years", time.to_string());

    // Test years, months, days combination
    let time = GuessTime {
        years: 5,
        months: 3,
        days: 10,
    };
    assert_eq!("5 years 3 months 10 days", time.to_string());

    // Test months and days only
    let time = GuessTime {
        years: 0,
        months: 6,
        days: 15,
    };
    assert_eq!("6 months 15 days", time.to_string());

    // Test days only
    let time = GuessTime {
        years: 0,
        months: 0,
        days: 20,
    };
    assert_eq!("20 days", time.to_string());

    // Test less than a day
    let time = GuessTime {
        years: 0,
        months: 0,
        days: 0,
    };
    assert_eq!("less than a day", time.to_string());
}

#[test]
fn test_guess_time_for_entropy_edge_cases() {
    // Test entropy > 64 (more than a billion years)
    let time = GuessTime::for_entropy(65);
    assert_eq!(1_000_000_001, time.years);

    // Test exact boundary at 64 (should still be > 64 branch)
    let time = GuessTime::for_entropy(64);
    assert_eq!(1_000_001, time.years); // Falls into > 54 branch, not > 64

    // Test entropy 55-64 (more than a million years)
    let time = GuessTime::for_entropy(55);
    assert_eq!(1_000_001, time.years);

    // Test exact boundary at 54 (should fall into > 44 branch)
    let time = GuessTime::for_entropy(54);
    assert_eq!(1001, time.years);

    // Test entropy 45-54 (more than a thousand years)
    let time = GuessTime::for_entropy(45);
    assert_eq!(1001, time.years);

    // Test exact boundary at 44 (should do actual calculation)
    let time = GuessTime::for_entropy(44);
    assert!(time.years < 1001);

    // Test lower entropy values (actual calculation)
    let time = GuessTime::for_entropy(30);
    assert!(time.years < 1000);
}

#[test]
fn test_entropy_display() {
    // Test when blind_min == blind_max
    let entropy = Entropy {
        blind_min: 50,
        blind_max: 50,
        seen: 40,
        guess_time: GuessTime {
            years: 100,
            months: 0,
            days: 0,
        },
    };
    let display = entropy.to_string();
    assert!(display.contains("50 bits blind"));
    assert!(display.contains("40 bits with full knowledge"));

    // Test when blind_min != blind_max
    let entropy = Entropy {
        blind_min: 45,
        blind_max: 55,
        seen: 35,
        guess_time: GuessTime {
            years: 0,
            months: 6,
            days: 0,
        },
    };
    let display = entropy.to_string();
    assert!(display.contains("between 45 & 55 bits"));
    assert!(display.contains("35 bits with full knowledge"));
}
