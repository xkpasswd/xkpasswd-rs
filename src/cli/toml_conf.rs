use super::*;
use crate::bit_flags::*;
use clap::ValueEnum;
use std::fs;

const CONFIG_FILE_NAME: &str = "xkpasswd.toml";

#[derive(Debug)]
pub enum ConfigParseError {
    Ignore,
    InvalidFile(String),
    InvalidConfig(String, String),
}

pub trait ConfigParser {
    fn parse_config_file(&mut self) -> Result<(), ConfigParseError>;
}

impl ConfigParser for Cli {
    fn parse_config_file(&mut self) -> Result<(), ConfigParseError> {
        let config = read_config_file(&self.config_file)?;

        parse_number_config(
            self.words_count.is_some(),
            &config,
            "words_count",
            |value| self.words_count = Some(value as u8),
        );

        parse_number_config(
            self.word_length_min.is_some(),
            &config,
            "word_min",
            |value| self.word_length_min = Some(value as u8),
        );

        parse_number_config(
            self.word_length_max.is_some(),
            &config,
            "word_max",
            |value| self.word_length_max = Some(value as u8),
        );

        parse_transforms(self.word_transforms.is_some(), &config, |transforms| {
            self.word_transforms = Some(transforms)
        })?;

        parse_str_config(self.separators.is_some(), &config, "separators", |value| {
            self.separators = Some(value)
        });

        parse_number_config(
            self.padding_digits_before.is_some(),
            &config,
            "digits_before",
            |value| self.padding_digits_before = Some(value as u8),
        );

        parse_number_config(
            self.padding_digits_after.is_some(),
            &config,
            "digits_after",
            |value| self.padding_digits_after = Some(value as u8),
        );

        parse_str_config(
            self.padding_symbols.is_some(),
            &config,
            "symbols",
            |value| self.padding_symbols = Some(value),
        );

        parse_number_config(
            self.padding_symbols_before.is_some(),
            &config,
            "symbols_before",
            |value| self.padding_symbols_before = Some(value as u8),
        );

        parse_number_config(
            self.padding_symbols_after.is_some(),
            &config,
            "symbols_after",
            |value| self.padding_symbols_after = Some(value as u8),
        );

        parse_enum_config(self.padding.is_some(), &config, "padding", |value| {
            self.padding = Some(value)
        })?;

        parse_enum_config(self.preset.is_some(), &config, "preset", |value| {
            self.preset = Some(value)
        })?;

        parse_enum_config(self.language.is_some(), &config, "lang", |value| {
            self.language = Some(value)
        })?;

        Ok(())
    }
}

fn lookup_default_config_path() -> Option<String> {
    for mut path in [dirs::preference_dir(), dirs::config_dir(), dirs::home_dir()]
        .into_iter()
        .flatten()
    {
        path.push(CONFIG_FILE_NAME);

        if path.exists() {
            return path.into_os_string().into_string().ok();
        }
    }

    None
}

fn read_config_file(config_file: &Option<String>) -> Result<toml::Value, ConfigParseError> {
    let file_data = match config_file {
        Some(config_file) => match fs::read_to_string(config_file) {
            Ok(data) => {
                log::debug!("found config file at custom path {}", config_file);
                Ok(data)
            }
            Err(err) => Err(ConfigParseError::InvalidFile(err.to_string())),
        },
        None => match lookup_default_config_path() {
            None => {
                log::debug!("config file at default path not found, ignoring");
                Err(ConfigParseError::Ignore)
            }
            Some(config_path) => match fs::read_to_string(&config_path) {
                Ok(data) => {
                    log::debug!("found config file at default path {}", config_path);
                    Ok(data)
                }
                Err(err) => Err(ConfigParseError::InvalidFile(err.to_string())),
            },
        },
    };

    match file_data {
        Err(err) => Err(err),
        Ok(data) => match toml::from_str::<toml::Value>(&data) {
            Err(parse_err) => Err(ConfigParseError::InvalidFile(parse_err.to_string())),
            Ok(parsed_data) => Ok(parsed_data),
        },
    }
}

fn parse_enum_config<T: ValueEnum, F: FnMut(T)>(
    ignore: bool,
    config: &toml::Value,
    field: &str,
    mut callback: F,
) -> Result<(), ConfigParseError> {
    if ignore {
        log::debug!("loading '{}' from command arguments", field);
        return Ok(());
    }

    match config.get_str(field) {
        Some(value_str) => match T::from_str(value_str, true) {
            Ok(value) => {
                callback(value);
                log::debug!("loading '{}' from config file", field);
                Ok(())
            }
            Err(err) => Err(ConfigParseError::InvalidConfig(field.to_string(), err)),
        },
        None => {
            log::debug!("loading default value for '{}'", field);
            Ok(())
        }
    }
}

fn parse_transforms<F: FnMut(Vec<WordTransform>)>(
    ignore: bool,
    config: &toml::Value,
    mut callback: F,
) -> Result<(), ConfigParseError> {
    let field = "transforms";

    if ignore {
        log::debug!("loading '{}' from command arguments", field);
        return Ok(());
    }

    let raw_transforms = match config.get_str_arr(field) {
        Err(err) => {
            return match err {
                ConfigParseError::Ignore => Ok(()),
                _ => Err(err),
            }
        }
        Ok(transforms) => transforms,
    };

    let parsed_transforms: Result<Vec<WordTransform>, String> = raw_transforms
        .iter()
        .map(|transform| WordTransform::from_str(transform, true))
        .collect();

    match parsed_transforms {
        Ok(word_transforms) => {
            log::debug!("loading '{}' from config file", field);
            callback(word_transforms);
            Ok(())
        }
        Err(err) => Err(ConfigParseError::InvalidConfig(
            "transforms".to_string(),
            err,
        )),
    }
}

fn parse_str_config<F: FnMut(String)>(
    ignore: bool,
    config: &toml::Value,
    field: &str,
    mut callback: F,
) {
    if ignore {
        log::debug!("loading '{}' from command arguments", field);
        return;
    }

    match config.get_str(field) {
        Some(value) => {
            callback(value.to_string());
            log::debug!("loading '{}' from config file", field);
        }
        None => log::debug!("loading default value for '{}'", field),
    }
}

fn parse_number_config<F: FnMut(u64)>(
    ignore: bool,
    config: &toml::Value,
    field: &str,
    mut callback: F,
) {
    if ignore {
        log::debug!("loading '{}' from command arguments", field);
        return;
    }

    match config.get_number(field) {
        Some(value) => {
            callback(value);
            log::debug!("loading '{}' from config file", field);
        }
        None => log::debug!("loading default value for '{}'", field),
    }
}

trait Getter {
    fn get_str_arr<'a>(&'a self, field: &'a str) -> Result<Vec<&'a str>, ConfigParseError>;
    fn get_array<'a>(&'a self, field: &'a str) -> Option<&'a Vec<toml::Value>>;
    fn get_str<'a>(&'a self, field: &str) -> Option<&'a str>;
    fn get_number(&self, field: &str) -> Option<u64>;
}

impl Getter for toml::Value {
    fn get_str_arr<'a>(&'a self, field: &'a str) -> Result<Vec<&'a str>, ConfigParseError> {
        match self.get_array(field) {
            Some(transforms) => {
                let result: Result<Vec<&str>, String> = transforms
                    .iter()
                    .map(|transform| {
                        transform.as_str().ok_or(format!(
                            "Invalid data type, expect string but got '{}'",
                            transform
                        ))
                    })
                    .collect();

                match result {
                    Ok(values) => Ok(values),
                    Err(message) => {
                        Err(ConfigParseError::InvalidConfig(field.to_string(), message))
                    }
                }
            }
            None => Err(ConfigParseError::Ignore),
        }
    }

    fn get_array(&self, field: &str) -> Option<&Vec<toml::Value>> {
        self.get(field)?.as_array()
    }

    fn get_str(&self, field: &str) -> Option<&str> {
        self.get(field)?.as_str()
    }

    fn get_number(&self, field: &str) -> Option<u64> {
        self.get(field)?
            .as_integer()
            .map(|data| data.unsigned_abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_preset_config() {
        let table = [
            (Preset::Default, "default"),
            (Preset::AppleID, "apple-id"),
            (Preset::WindowsNtlmV1, "ntlm"),
            (Preset::SecurityQuestions, "secq"),
            (Preset::Web16, "web16"),
            (Preset::Web32, "web32"),
            (Preset::Wifi, "wifi"),
            (Preset::Xkcd, "xkcd"),
        ];

        for (_preset, config_value) in table {
            let config: toml::Value =
                toml::from_str(format!(r#"preset = "{}""#, config_value).as_str()).unwrap();

            let result = parse_enum_config(true, &config, "foo", |_: Preset| {
                panic!("shouldn't be invoked")
            });
            assert!(matches!(result, Ok(())));

            let result = parse_enum_config(false, &config, "foo", |_: Preset| {
                panic!("shouldn't be invoked")
            });
            assert!(matches!(result, Ok(())));

            let result = parse_enum_config(false, &config, "preset", |value: Preset| {
                assert!(matches!(value, _preset))
            });
            assert!(matches!(result, Ok(())));
        }

        let config: toml::Value = toml::from_str(r#"preset = "apple_id""#).unwrap();
        let result = parse_enum_config(false, &config, "preset", |_: Preset| {
            panic!("shouldn't be invoked")
        });

        if let ConfigParseError::InvalidConfig(field, message) = result.err().unwrap() {
            assert_eq!("preset", field);
            assert_eq!("invalid variant: apple_id", message);
        } else {
            panic!("shouldn't be invoked")
        }
    }

    #[test]
    fn test_parse_padding_config() {
        let table = [
            (CliPadding::Fixed, "fixed"),
            (CliPadding::Adaptive, "adaptive"),
        ];

        for (_preset, config_value) in table {
            let config: toml::Value =
                toml::from_str(format!(r#"padding = "{}""#, config_value).as_str()).unwrap();

            let result = parse_enum_config(true, &config, "foo", |_: CliPadding| {
                panic!("shouldn't be invoked")
            });
            assert!(matches!(result, Ok(())));

            let result = parse_enum_config(false, &config, "foo", |_: CliPadding| {
                panic!("shouldn't be invoked")
            });
            assert!(matches!(result, Ok(())));

            let result = parse_enum_config(false, &config, "preset", |value: CliPadding| {
                assert!(matches!(value, _preset))
            });
            assert!(matches!(result, Ok(())));
        }

        let config: toml::Value = toml::from_str(r#"padding = "fixed_padding""#).unwrap();
        let result = parse_enum_config(false, &config, "padding", |_: CliPadding| {
            panic!("shouldn't be invoked")
        });

        if let ConfigParseError::InvalidConfig(field, message) = result.err().unwrap() {
            assert_eq!("padding", field);
            assert_eq!("invalid variant: fixed_padding", message);
        } else {
            panic!("shouldn't be invoked")
        }
    }

    #[test]
    fn test_parse_transforms() {
        let config: toml::Value = toml::from_str(r#"transforms = ["lowercase"]"#).unwrap();
        let result = parse_transforms(true, &config, |_| panic!("shouldn't be invoked"));
        assert!(matches!(result, Ok(())));

        let result = parse_transforms(false, &config, |value| {
            assert_eq!(vec![WordTransform::Lowercase], value)
        });
        assert!(matches!(result, Ok(())));

        let config: toml::Value = toml::from_str(r#"transforms = "lowercase""#).unwrap();
        let result = parse_transforms(false, &config, |_| panic!("shouldn't be invoked"));
        assert!(matches!(result, Ok(())));

        let config: toml::Value = toml::from_str(r#"transforms = ["lowercase", false]"#).unwrap();
        let result = parse_transforms(false, &config, |_| panic!("shouldn't be invoked"));
        if let ConfigParseError::InvalidConfig(field, message) = result.err().unwrap() {
            assert_eq!("transforms", field);
            assert_eq!("Invalid data type, expect string but got 'false'", message);
        } else {
            panic!("shouldn't be invoked")
        }

        let config: toml::Value =
            toml::from_str(r#"transforms = ["lowercase", "inversed_titlecase"]"#).unwrap();
        let result = parse_transforms(false, &config, |_| panic!("shouldn't be invoked"));
        if let ConfigParseError::InvalidConfig(field, message) = result.err().unwrap() {
            assert_eq!("transforms", field);
            assert_eq!("invalid variant: inversed_titlecase", message);
        } else {
            panic!("shouldn't be invoked")
        }
    }

    #[test]
    fn test_parse_str_config() {
        let config: toml::Value = toml::from_str(r#"separators = "!@#""#).unwrap();
        parse_str_config(true, &config, "foo", |_| panic!("shouldn't be invoked"));
        parse_str_config(false, &config, "foo", |_| panic!("shouldn't be invoked"));
        parse_str_config(false, &config, "separators", |value| {
            assert_eq!("!@#", value)
        });

        let config: toml::Value = toml::from_str(r#"separators = false"#).unwrap();
        parse_str_config(false, &config, "separators", |_| {
            panic!("shouldn't be invoked")
        });
    }

    #[test]
    fn test_parse_number_config() {
        let config: toml::Value = toml::from_str(r#"words_count = 3"#).unwrap();
        parse_number_config(true, &config, "foo", |_| panic!("shouldn't be invoked"));
        parse_number_config(false, &config, "foo", |_| panic!("shouldn't be invoked"));
        parse_number_config(false, &config, "words_count", |value| assert_eq!(3, value));

        let config: toml::Value = toml::from_str(r#"words_count = true"#).unwrap();
        parse_number_config(false, &config, "words_count", |_| {
            panic!("shouldn't be invoked")
        });
    }

    #[test]
    fn test_parse_config_file_with_temp_file() {
        // Create a temp config file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
words_count = 5
word_min = 4
word_max = 8
separators = ".-_"
digits_before = 2
digits_after = 3
symbols = "!@#$"
symbols_before = 1
symbols_after = 2
preset = "web32"
lang = "de"
transforms = ["lowercase", "uppercase"]
"#
        )
        .unwrap();

        let mut cli = Cli {
            config_file: Some(temp_file.path().to_str().unwrap().to_string()),
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
        };

        let result = cli.parse_config_file();
        assert!(result.is_ok());

        // Verify values were parsed from config
        assert_eq!(Some(5), cli.words_count);
        assert_eq!(Some(4), cli.word_length_min);
        assert_eq!(Some(8), cli.word_length_max);
        assert_eq!(Some(".-_".to_string()), cli.separators);
        assert_eq!(Some(2), cli.padding_digits_before);
        assert_eq!(Some(3), cli.padding_digits_after);
        assert_eq!(Some("!@#$".to_string()), cli.padding_symbols);
        assert_eq!(Some(1), cli.padding_symbols_before);
        assert_eq!(Some(2), cli.padding_symbols_after);
        assert!(matches!(cli.preset, Some(Preset::Web32)));
        assert!(matches!(cli.language, Some(Language::German)));
        assert_eq!(
            Some(vec![WordTransform::Lowercase, WordTransform::Uppercase]),
            cli.word_transforms
        );
    }

    #[test]
    fn test_parse_config_file_cli_overrides_config() {
        // Create a temp config file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"words_count = 5"#).unwrap();

        // CLI already has words_count set - should not be overridden
        let mut cli = Cli {
            config_file: Some(temp_file.path().to_str().unwrap().to_string()),
            words_count: Some(10), // CLI value should take precedence
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
        };

        let result = cli.parse_config_file();
        assert!(result.is_ok());

        // CLI value should be preserved (not overridden by config)
        assert_eq!(Some(10), cli.words_count);
    }

    #[test]
    fn test_parse_config_file_invalid_file() {
        let mut cli = Cli {
            config_file: Some("/nonexistent/path/to/config.toml".to_string()),
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
        };

        let result = cli.parse_config_file();
        assert!(matches!(result, Err(ConfigParseError::InvalidFile(_))));
    }

    #[test]
    fn test_parse_config_file_invalid_toml() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "invalid toml {{ content").unwrap();

        let mut cli = Cli {
            config_file: Some(temp_file.path().to_str().unwrap().to_string()),
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
        };

        let result = cli.parse_config_file();
        assert!(matches!(result, Err(ConfigParseError::InvalidFile(_))));
    }

    #[test]
    fn test_parse_config_file_no_config() {
        // No config file specified - behavior depends on whether user has a default config
        // This test verifies that when no explicit config is given, we either:
        // - Return Ignore if no default config exists
        // - Return Ok if a default config exists and is valid
        let mut cli = Cli {
            config_file: None,
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
        };

        let result = cli.parse_config_file();
        // Either no config found (Ignore) or config found and parsed (Ok)
        // We don't fail on InvalidFile/InvalidConfig here
        assert!(matches!(result, Err(ConfigParseError::Ignore) | Ok(())));
    }

    #[test]
    fn test_parse_config_file_type_mismatch() {
        // Test that wrong types in config are handled gracefully
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
words_count = "five"
separators = 123
"#
        )
        .unwrap();

        let mut cli = Cli {
            config_file: Some(temp_file.path().to_str().unwrap().to_string()),
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
        };

        // Should succeed - type mismatches are silently ignored (use defaults)
        let result = cli.parse_config_file();
        assert!(result.is_ok());

        // Values should remain None since types didn't match
        assert_eq!(None, cli.words_count);
        assert_eq!(None, cli.separators);
    }

    #[test]
    fn test_parse_config_file_with_padding_strategy() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"padding = "adaptive""#).unwrap();

        let mut cli = Cli {
            config_file: Some(temp_file.path().to_str().unwrap().to_string()),
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
        };

        let result = cli.parse_config_file();
        assert!(result.is_ok());
        assert!(matches!(cli.padding, Some(CliPadding::Adaptive)));
    }
}
