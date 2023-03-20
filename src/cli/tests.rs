use super::*;
use crate::settings::*;

const DEFAULT_CLI: Cli = Cli {
    words_count: None,
    word_length_min: None,
    word_length_max: None,
    word_transforms: None,
    separators: None,
    padding_digits_before: None,
    padding_digits_after: None,
    padding_symbols: None,
    padding_symbols_before: None,
    padding_symbols_after: None,
    padding: None,
    adaptive_length: None,
    preset: None,
    verbosity: 0,
    language: None,
    config_file: None,
};

#[test]
#[should_panic]
fn test_build_settings_panic() {
    let cli = Cli {
        words_count: Some(0),
        ..DEFAULT_CLI
    };
    let _: Settings = cli.build_settings().unwrap();
}

#[test]
fn test_build_settings_default() {
    let settings: Settings = DEFAULT_CLI.build_settings().unwrap();
    assert_eq!(Settings::default(), settings);
}

#[test]
fn test_build_settings_from_preset() {
    let presets = [
        Preset::Default,
        Preset::AppleID,
        Preset::WindowsNtlmV1,
        Preset::SecurityQuestions,
        Preset::Web16,
        Preset::Web32,
        Preset::Wifi,
        Preset::Xkcd,
    ];

    for preset in presets {
        let cli = Cli {
            preset: Some(preset),
            ..DEFAULT_CLI
        };

        let settings: Settings = cli.build_settings().unwrap();
        assert_eq!(Settings::from_preset(preset), settings);
    }
}

#[test]
fn test_build_settings_custom() {
    let cli = Cli {
        words_count: Some(5),
        word_length_min: Some(5),
        word_length_max: Some(6),
        word_transforms: Some(vec![
            WordTransform::Lowercase,
            WordTransform::InversedTitlecase,
        ]),
        separators: Some("~@#".to_string()),
        padding_digits_before: Some(1),
        padding_digits_after: Some(3),
        padding_symbols: Some("$%^".to_string()),
        padding_symbols_before: Some(3),
        padding_symbols_after: Some(1),
        padding: Some(CliPadding::Adaptive),
        adaptive_length: Some(17),
        ..DEFAULT_CLI
    };

    let expected_settings = Settings::default()
        .with_words_count(5)
        .unwrap()
        .with_word_lengths(Some(5), Some(6))
        .unwrap()
        .with_word_transforms(WordTransform::Lowercase | WordTransform::InversedTitlecase)
        .unwrap()
        .with_separators("~@#")
        .with_padding_digits(Some(1), Some(3))
        .with_padding_symbols("$%^")
        .with_padding_symbol_lengths(Some(3), Some(1))
        .with_padding_strategy(PaddingStrategy::Adaptive(17))
        .unwrap();

    assert_eq!(expected_settings, cli.build_settings::<Settings>().unwrap());
}
