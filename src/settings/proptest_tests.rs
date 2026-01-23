use super::*;
use crate::error::XkpasswdError;
use proptest::prelude::*;
use std::cmp;

proptest! {
    // 1. Words count validation tests
    #[test]
    fn words_count_valid_range(count in 1u8..=255) {
        let result = Settings::default().with_words_count(count);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap().words_count, count);
    }

    #[test]
    fn words_count_zero_fails(count in 0u8..=0) {
        let result = Settings::default().with_words_count(count);
        prop_assert!(result.is_err());
        prop_assert!(matches!(result.unwrap_err(), XkpasswdError::InvalidWordsCount));
    }

    // 2. Word lengths validation tests
    #[test]
    fn word_lengths_valid_ranges(
        min in Settings::MIN_WORD_LENGTH..=Settings::MAX_WORD_LENGTH,
        max in Settings::MIN_WORD_LENGTH..=Settings::MAX_WORD_LENGTH
    ) {
        let result = Settings::default().with_word_lengths(Some(min), Some(max));
        prop_assert!(result.is_ok());

        let settings = result.unwrap();
        let expected_min = cmp::min(min, max);
        let expected_max = cmp::max(min, max);
        prop_assert_eq!(settings.word_lengths, (expected_min, expected_max));
    }

    #[test]
    fn word_lengths_swapped_corrects(
        min in (Settings::MIN_WORD_LENGTH + 1)..=Settings::MAX_WORD_LENGTH,
        max in Settings::MIN_WORD_LENGTH..(Settings::MAX_WORD_LENGTH)
    ) {
        prop_assume!(min > max); // Ensure they're swapped

        let result = Settings::default().with_word_lengths(Some(min), Some(max));
        prop_assert!(result.is_ok());

        let settings = result.unwrap();
        // Should be corrected: max becomes min, min becomes max
        prop_assert_eq!(settings.word_lengths, (max, min));
    }

    #[test]
    fn word_lengths_min_too_low_fails(min in 0u8..(Settings::MIN_WORD_LENGTH)) {
        let result = Settings::default().with_word_lengths(Some(min), Some(Settings::MAX_WORD_LENGTH));
        prop_assert!(result.is_err());
        prop_assert!(matches!(result.unwrap_err(), XkpasswdError::MinWordLengthTooSmall));
    }

    #[test]
    fn word_lengths_max_too_high_fails(max in (Settings::MAX_WORD_LENGTH + 1)..=255u8) {
        let result = Settings::default().with_word_lengths(Some(Settings::MIN_WORD_LENGTH), Some(max));
        prop_assert!(result.is_err());
        prop_assert!(matches!(result.unwrap_err(), XkpasswdError::MaxWordLengthTooLarge));
    }

    // 3. Transform validation tests
    #[test]
    fn single_lowercase_transform_valid(_dummy in 0u8..1) {
        let result = Settings::default()
            .with_word_transforms(FieldSize::from_flag(WordTransform::Lowercase));
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap().word_transforms, FieldSize::from_flag(WordTransform::Lowercase));
    }

    #[test]
    fn single_uppercase_transform_valid(_dummy in 0u8..1) {
        let result = Settings::default()
            .with_word_transforms(FieldSize::from_flag(WordTransform::Uppercase));
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap().word_transforms, FieldSize::from_flag(WordTransform::Uppercase));
    }

    #[test]
    fn combined_transforms_valid(_dummy in 0u8..1) {
        let combined = WordTransform::Lowercase | WordTransform::Uppercase;
        let result = Settings::default().with_word_transforms(combined);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap().word_transforms, combined);
    }

    #[test]
    fn group_transform_overrides_singles(_dummy in 0u8..1) {
        let combined_with_group = WordTransform::Lowercase | WordTransform::AltercaseUpperFirst;
        let result = Settings::default().with_word_transforms(combined_with_group);
        prop_assert!(result.is_ok());
        // Group transform should override singles
        prop_assert_eq!(result.unwrap().word_transforms, FieldSize::from_flag(WordTransform::AltercaseUpperFirst));
    }

    #[test]
    fn invalid_transform_fails(_dummy in 0u8..1) {
        let result = Settings::default().with_word_transforms(0b00010000);
        prop_assert!(result.is_err());
        prop_assert!(matches!(result.unwrap_err(), XkpasswdError::InvalidTransform));
    }

    // 4. Padding strategy tests
    #[test]
    fn fixed_padding_always_succeeds(_dummy in 0u8..1) {
        let result = Settings::default().with_padding_strategy(PaddingStrategy::Fixed);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap().padding_strategy, PaddingStrategy::Fixed);
    }

    #[test]
    fn adaptive_zero_fails(_dummy in 0u8..1) {
        let result = Settings::default().with_padding_strategy(PaddingStrategy::Adaptive(0));
        prop_assert!(result.is_err());
        prop_assert!(matches!(result.unwrap_err(), XkpasswdError::InvalidAdaptivePadding));
    }

    #[test]
    fn adaptive_positive_succeeds(size in 1usize..1000) {
        let result = Settings::default().with_padding_strategy(PaddingStrategy::Adaptive(size));
        prop_assert!(result.is_ok());
        let settings = result.unwrap();
        prop_assert_eq!(settings.padding_strategy, PaddingStrategy::Adaptive(size));
        // Should reset padding symbol lengths to (0, 0)
        prop_assert_eq!(settings.padding_symbol_lengths, (0, 0));
    }

    // 5. Entropy calculation determinism tests
    #[test]
    fn entropy_calculation_deterministic(
        words_count in 1u8..=10,
        pool_size in 100usize..10000,
        min_len in Settings::MIN_WORD_LENGTH..Settings::MAX_WORD_LENGTH,
        max_len in Settings::MIN_WORD_LENGTH..Settings::MAX_WORD_LENGTH
    ) {
        let min = cmp::min(min_len, max_len);
        let max = cmp::max(min_len, max_len);

        let settings = Settings::default()
            .with_words_count(words_count).unwrap()
            .with_word_lengths(Some(min), Some(max)).unwrap();

        let entropy1 = settings.calc_entropy(pool_size);
        let entropy2 = settings.calc_entropy(pool_size);

        prop_assert_eq!(entropy1, entropy2, "Entropy calculation should be deterministic");
    }

    // 6. Password generation properties tests
    #[test]
    fn generated_password_has_correct_word_count(words_count in 1u8..=5) {
        let settings = Settings::default()
            .with_words_count(words_count)
            .unwrap();

        // Test pool with enough words
        let test_pool: Vec<&str> = (0..words_count * 2)
            .map(|i| match i % 4 {
                0 => "test",
                1 => "word",
                2 => "pool",
                _ => "item"
            })
            .collect();

        let generated_words = settings.rand_words(&test_pool);
        prop_assert_eq!(generated_words.len(), words_count as usize);
    }

    #[test]
    fn lowercase_transform_applied_correctly(_dummy in 0u8..1) {
        let settings = Settings::default()
            .with_words_count(1).unwrap()
            .with_word_transforms(FieldSize::from_flag(WordTransform::Lowercase)).unwrap();

        let test_pool = ["TestWord"];
        let generated_words = settings.rand_words(&test_pool);

        prop_assert_eq!(generated_words.len(), 1);
        prop_assert_eq!(&generated_words[0], "testword");
    }

    #[test]
    fn uppercase_transform_applied_correctly(_dummy in 0u8..1) {
        let settings = Settings::default()
            .with_words_count(1).unwrap()
            .with_word_transforms(FieldSize::from_flag(WordTransform::Uppercase)).unwrap();

        let test_pool = ["TestWord"];
        let generated_words = settings.rand_words(&test_pool);

        prop_assert_eq!(generated_words.len(), 1);
        prop_assert_eq!(&generated_words[0], "TESTWORD");
    }

    #[test]
    fn altercase_lower_first_pattern_correct(words_count in 2u8..=6) {
        let settings = Settings::default()
            .with_words_count(words_count).unwrap()
            .with_word_transforms(FieldSize::from_flag(WordTransform::AltercaseLowerFirst)).unwrap();

        let test_pool = vec!["word"; words_count as usize * 2]; // Enough unique positions
        let generated_words = settings.rand_words(&test_pool);

        prop_assert_eq!(generated_words.len(), words_count as usize);

        // Check alternating pattern - should start with lowercase (index 0)
        for (idx, word) in generated_words.iter().enumerate() {
            if idx % 2 == 0 {
                prop_assert_eq!(word, "word", "Even indices should be lowercase");
            } else {
                prop_assert_eq!(word, "WORD", "Odd indices should be uppercase");
            }
        }
    }

    #[test]
    fn altercase_upper_first_pattern_correct(words_count in 2u8..=6) {
        let settings = Settings::default()
            .with_words_count(words_count).unwrap()
            .with_word_transforms(FieldSize::from_flag(WordTransform::AltercaseUpperFirst)).unwrap();

        let test_pool = vec!["word"; words_count as usize * 2]; // Enough unique positions
        let generated_words = settings.rand_words(&test_pool);

        prop_assert_eq!(generated_words.len(), words_count as usize);

        // Check alternating pattern - should start with uppercase (index 0)
        for (idx, word) in generated_words.iter().enumerate() {
            if idx % 2 == 0 {
                prop_assert_eq!(word, "WORD", "Even indices should be uppercase");
            } else {
                prop_assert_eq!(word, "word", "Odd indices should be lowercase");
            }
        }
    }

    // Additional property tests for edge cases and invariants
    #[test]
    fn settings_immutability_preserved(
        words_count1 in 1u8..=10,
        words_count2 in 1u8..=10
    ) {
        let original = Settings::default().with_words_count(words_count1).unwrap();
        let modified = original.with_words_count(words_count2).unwrap();

        // Original should be unchanged
        prop_assert_eq!(original.words_count, words_count1);
        // New instance should have new value
        prop_assert_eq!(modified.words_count, words_count2);
    }

    #[test]
    fn word_lengths_range_consistency(
        min in Settings::MIN_WORD_LENGTH..=Settings::MAX_WORD_LENGTH,
        max in Settings::MIN_WORD_LENGTH..=Settings::MAX_WORD_LENGTH
    ) {
        let settings = Settings::default()
            .with_word_lengths(Some(min), Some(max)).unwrap();

        let range = settings.word_lengths();
        let expected_min = cmp::min(min, max);
        let expected_max = cmp::max(min, max);

        prop_assert_eq!(range.start, expected_min);
        prop_assert_eq!(range.end, expected_max + 1); // Range is exclusive at end
    }

    // Separator and padding tests
    #[test]
    fn separator_customization_works(_dummy in 0u8..1) {
        let separator = "-";
        let settings = Settings::default().with_separators(separator);

        let random_separator = settings.rand_separator();
        prop_assert_eq!(random_separator.len(), 1);

        // Test that the setting was stored correctly
        prop_assert_eq!(settings.separators, separator);
        // Test that the random separator is from the configured set
        prop_assert!(separator.contains(&random_separator));
    }

    #[test]
    fn padding_digits_configuration_works(
        prefix in 0u8..5,
        suffix in 0u8..5
    ) {
        let settings = Settings::default()
            .with_padding_digits(Some(prefix), Some(suffix));

        prop_assert_eq!(settings.padding_digits, (prefix, suffix));

        let (_prefix_symbols, prefix_digits_str) = settings.rand_prefix();
        let (suffix_digits_str, _suffix_symbols) = settings.rand_suffix();

        if prefix > 0 {
            prop_assert_eq!(prefix_digits_str.len(), prefix as usize);
            // Should be parseable as a number
            prop_assert!(prefix_digits_str.parse::<u64>().is_ok());
        } else {
            prop_assert_eq!(prefix_digits_str.len(), 0);
        }

        if suffix > 0 {
            prop_assert_eq!(suffix_digits_str.len(), suffix as usize);
            // Should be parseable as a number
            prop_assert!(suffix_digits_str.parse::<u64>().is_ok());
        } else {
            prop_assert_eq!(suffix_digits_str.len(), 0);
        }
    }

    // Test entropy bounds are sensible
    #[test]
    fn entropy_values_sensible(
        words_count in 1u8..=5,
        pool_size in 1000usize..5000
    ) {
        let settings = Settings::default()
            .with_words_count(words_count).unwrap();

        let entropy = settings.calc_entropy(pool_size);

        // Basic sanity checks on entropy values
        prop_assert!(entropy.blind_min > 0, "Blind min entropy should be positive");
        prop_assert!(entropy.blind_max >= entropy.blind_min, "Blind max should be >= blind min");
        prop_assert!(entropy.seen > 0, "Seen entropy should be positive");

        // More words should generally mean more entropy (with same pool)
        if words_count > 1 {
            let single_word_settings = Settings::default().with_words_count(1).unwrap();
            let single_entropy = single_word_settings.calc_entropy(pool_size);
            prop_assert!(entropy.seen > single_entropy.seen, "More words should mean more entropy");
        }
    }
}
