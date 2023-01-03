use super::*;
use std::collections::HashSet;

#[test]
fn test_default_settings() {
    let settings = Settings::default();

    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_digits
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_symbol_lengths
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));
}

#[test]
fn test_with_words_count() {
    let _err: Result<Settings, String> =
        Err("only positive integer is allowed for words count".to_string());

    // invalid value
    assert!(matches!(Settings::default().with_words_count(0), _err));

    let settings = Settings::default().with_words_count(1).unwrap();
    // only words_count updated
    assert_eq!(1, settings.words_count);

    // other fields remain unchanged
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_digits
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_symbol_lengths
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));

    // overriding with multiple calls
    let other_settings = settings.with_words_count(123).unwrap();
    assert_eq!(123, other_settings.words_count);
}

#[test]
fn test_with_word_lengths() {
    let _err: Result<Settings, String> = Err(MIN_WORD_LENGTH_ERR.to_string());

    // invalid lengths
    assert!(matches!(
        Settings::default().with_word_lengths(
            Some(Settings::MIN_WORD_LENGTH - 1),
            Some(Settings::MAX_WORD_LENGTH + 1)
        ),
        _err
    ));

    let _err: Result<Settings, String> = Err(MAX_WORD_LENGTH_ERR.to_string());

    // max word length has lower priority
    assert!(matches!(
        Settings::default().with_word_lengths(
            Some(Settings::MIN_WORD_LENGTH),
            Some(Settings::MAX_WORD_LENGTH + 1)
        ),
        _err
    ));

    let settings = Settings::default()
        .with_word_lengths(Some(4), Some(6))
        .unwrap();
    // only word_lengths updated
    assert_eq!((4, 6), settings.word_lengths);

    // other fields remain unchanged
    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_symbol_lengths
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));

    // overriding with multiple calls
    let other_settings = settings.with_word_lengths(Some(5), Some(5)).unwrap();
    assert_eq!((5, 5), other_settings.word_lengths); // equal values

    let other_settings = settings.with_word_lengths(Some(6), Some(4)).unwrap();
    assert_eq!((4, 6), other_settings.word_lengths); // min/max corrected

    // with None values
    let settings = Settings::default()
        .with_word_lengths(Some(4), Some(7))
        .unwrap();

    let other_settings = settings.with_word_lengths(None, Some(5)).unwrap();
    assert_eq!((4, 5), other_settings.word_lengths);

    let other_settings = settings.with_word_lengths(Some(8), None).unwrap();
    assert_eq!((7, 8), other_settings.word_lengths);
}

#[test]
fn test_with_separators() {
    let settings = Settings::default().with_separators("abc123");
    // only separators updated
    assert_eq!("abc123".to_string(), settings.separators);

    // other fields remain unchanged
    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_digits
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_symbol_lengths
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));

    // overriding with multiple calls
    let other_settings = settings.with_separators("");
    assert_eq!("".to_string(), other_settings.separators);
}

#[test]
fn test_with_padding_digits() {
    let settings = Settings::default().with_padding_digits(Some(1), Some(3));
    // only padding_digits updated
    assert_eq!((1, 3), settings.padding_digits);

    // other fields remain unchanged
    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_symbol_lengths
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));

    // overriding with multiple calls
    let other_settings = settings.with_padding_digits(Some(0), Some(0));
    assert_eq!((0, 0), other_settings.padding_digits);

    // with None values
    let settings = Settings::default().with_padding_digits(Some(2), Some(4));

    let other_settings = settings.with_padding_digits(None, Some(5));
    assert_eq!((2, 5), other_settings.padding_digits);

    let other_settings = settings.with_padding_digits(Some(8), None);
    assert_eq!((8, 4), other_settings.padding_digits);
}

#[test]
fn test_with_padding_symbols() {
    let settings = Settings::default().with_padding_symbols("456xyz");
    // only padding_symbols updated
    assert_eq!("456xyz".to_string(), settings.padding_symbols);

    // other fields remain unchanged
    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_digits
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_symbol_lengths
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));

    // overriding with multiple calls
    let other_settings = settings.with_padding_symbols("def789");
    assert_eq!("def789", other_settings.padding_symbols);
}

#[test]
fn test_with_padding_symbol_lengths() {
    let settings = Settings::default()
        .with_padding_strategy(PaddingStrategy::Adaptive(12))
        .unwrap()
        .with_padding_symbol_lengths(Some(3), Some(4));

    // only padding_symbol_lengths and padding_strategy updated
    assert_eq!((3, 4), settings.padding_symbol_lengths);
    assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));

    // other fields remain unchanged
    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_digits
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));

    // however, if the symbol lenghts are both None, don't touch padding_strategy
    let settings = Settings::default()
        .with_padding_strategy(PaddingStrategy::Adaptive(12))
        .unwrap()
        .with_padding_symbol_lengths(None, None);

    // only padding_symbol_lengths and padding_strategy updated
    assert_eq!((0, 0), settings.padding_symbol_lengths);
    assert!(matches!(
        settings.padding_strategy,
        PaddingStrategy::Adaptive(12)
    ));

    // overriding with multiple calls
    let other_settings = settings.with_padding_symbol_lengths(Some(0), Some(0));
    assert_eq!((0, 0), other_settings.padding_symbol_lengths);

    // with None values
    let settings = Settings::default().with_padding_symbol_lengths(Some(2), Some(4));

    let other_settings = settings.with_padding_symbol_lengths(None, Some(5));
    assert_eq!((2, 5), other_settings.padding_symbol_lengths);

    let other_settings = settings.with_padding_symbol_lengths(Some(8), None);
    assert_eq!((8, 4), other_settings.padding_symbol_lengths);
}

#[test]
fn test_with_padding_strategy() {
    let _err: Result<Settings, String> = Err("invalid adaptive padding number".to_string());

    // invalid adaptive padding
    assert!(matches!(
        Settings::default().with_padding_strategy(PaddingStrategy::Adaptive(0)),
        _err
    ));

    let settings = Settings::default()
        .with_padding_symbol_lengths(Some(2), Some(3))
        .with_padding_strategy(PaddingStrategy::Fixed)
        .unwrap();
    // only padding_strategy updated
    assert!(matches!(settings.padding_strategy, PaddingStrategy::Fixed));
    assert_eq!((2, 3), settings.padding_symbol_lengths);

    let settings = Settings::default()
        .with_padding_strategy(PaddingStrategy::Adaptive(16))
        .unwrap();
    // both padding_strategy and padding_symbol_lengths updated
    assert!(matches!(
        settings.padding_strategy,
        PaddingStrategy::Adaptive(16)
    ));
    assert_eq!((0, 0), settings.padding_symbol_lengths);

    // other fields remain unchanged
    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(Settings::DEFAULT_WORD_TRANSFORMS, settings.word_transforms);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_digits
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );

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
    assert_eq!(Settings::DEFAULT_WORDS_COUNT, settings.words_count);
    assert_eq!(Settings::DEFAULT_WORD_LENGTHS, settings.word_lengths);
    assert_eq!(
        Settings::DEFAULT_SEPARATORS.to_string(),
        settings.separators
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_digits
    );
    assert_eq!(
        Settings::DEFAULT_SYMBOLS.to_string(),
        settings.padding_symbols
    );
    assert_eq!(
        (0, Settings::DEFAULT_PADDING_LENGTH),
        settings.padding_symbol_lengths
    );
    assert!(matches!(
        settings.padding_strategy,
        Settings::DEFAULT_PADDING_STRATEGY
    ));

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
        WordTransform::AltercaseLowerFirst,
        WordTransform::AltercaseUpperFirst,
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
        let settings = Settings::default()
            .with_word_lengths(Some(min), Some(max))
            .unwrap();
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
            .with_padding_digits(Some(prefix_digits), Some(suffix_digits))
            .with_padding_symbol_lengths(Some(prefix_symbols), Some(suffix_symbols));
        let (symbols, digits) = settings.rand_prefix();
        assert_eq!("", symbols);
        assert_eq!("", digits);
    }

    for prefix_symbols in 1usize..10 {
        for prefix_digits in 1usize..10 {
            let settings = Settings::default()
                .with_padding_digits(Some(prefix_digits as u8), Some(2))
                .with_padding_symbols("#")
                .with_padding_symbol_lengths(Some(prefix_symbols as u8), Some(3));
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
            .with_padding_digits(Some(prefix_digits), Some(suffix_digits))
            .with_padding_symbol_lengths(Some(prefix_symbols), Some(suffix_symbols));
        let (digits, symbols) = settings.rand_suffix();
        assert_eq!("", digits);
        assert_eq!("", symbols);
    }

    for suffix_symbols in 1usize..10 {
        for suffix_digits in 1usize..10 {
            let settings = Settings::default()
                .with_padding_digits(Some(2), Some(suffix_digits as u8))
                .with_padding_symbols("~")
                .with_padding_symbol_lengths(Some(3), Some(suffix_symbols as u8));
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
        PaddingResult::TrimTo(10)
    ));
}

#[test]
fn test_calc_entropy() {
    let table = [
        ((Preset::AppleID, 4351), (164, 203, 57), (1_000_001, 0, 0)),
        ((Preset::WindowsNtlmV1, 1380), (92, 92, 31), (0, 0, 24)),
        (
            (Preset::SecurityQuestions, 6631),
            (176, 316, 78),
            (1_000_000_001, 0, 0),
        ),
        ((Preset::Web16, 1113), (102, 102, 40), (34, 10, 15)),
        ((Preset::Web32, 2493), (177, 203, 65), (1_000_000_001, 0, 0)),
        ((Preset::Wifi, 6631), (413, 413, 116), (1_000_000_001, 0, 0)),
        ((Preset::Xkcd, 6631), (121, 224, 55), (1_000_001, 0, 0)),
    ];

    for ((preset, pool_size), (blind_min, blind_max, seen), (years, months, days)) in table {
        let guess_time = GuessTime {
            years,
            months,
            days,
        };
        let expected = Entropy {
            blind_max,
            blind_min,
            seen,
            guess_time,
        };
        let entropy = Settings::from_preset(preset).calc_entropy(pool_size);
        assert_eq!(expected, entropy);
    }
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

        let unique_words: HashSet<String> = words.iter().map(|word| word.to_lowercase()).collect();
        assert!(unique_words.len() < 3);
    }

    // enough pool
    let pool = &["foo", "bar", "fooz", "barz"];

    for _ in 0..10 {
        let words = settings.build_words_list(pool);
        assert_eq!(3, words.len());

        let unique_words: HashSet<String> = words.iter().map(|word| word.to_lowercase()).collect();
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
            FieldSize::from_flag(WordTransform::AltercaseLowerFirst),
            vec![
                WordTransform::Lowercase,
                WordTransform::Uppercase,
                WordTransform::Lowercase,
            ],
        ),
        (
            FieldSize::from_flag(WordTransform::AltercaseUpperFirst),
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
        let result = rand_chars(Settings::DEFAULT_SYMBOLS, 1);
        assert!(Settings::DEFAULT_SYMBOLS.contains(&result));
    }

    // multi char randomize
    for _ in 0..10 {
        for count in 2..5 {
            let result = rand_chars(Settings::DEFAULT_SYMBOLS, count);
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
