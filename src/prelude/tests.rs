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
#[should_panic]
fn test_load_dict_invalid_data() {
    let dict_bytes = "foo:3".as_bytes();
    load_dict(dict_bytes);
}

#[cfg(feature = "lang_en")]
#[test]
fn test_xkpasswd_for_en() {
    let pass = Xkpasswd::for_language(Language::English);
    assert_eq!(false, pass.dict.is_empty());

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
    assert_eq!(false, pass.dict.is_empty());

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
    assert_eq!(false, pass.dict.is_empty());

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
    assert_eq!(false, pass.dict.is_empty());

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
    assert_eq!(false, pass.dict.is_empty());

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
