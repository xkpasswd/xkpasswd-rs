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
        let config = match read_config_file(&self.config_file) {
            Err(err) => return Err(err),
            Ok(config) => config,
        };

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
            return match path.into_os_string().into_string() {
                Ok(config_path) => Some(config_path),
                Err(_) => None,
            };
        }
    }

    None
}

fn read_config_file(config_file: &Option<String>) -> Result<toml::Value, ConfigParseError> {
    let file_data = match config_file {
        Some(config_file) => match fs::read(config_file) {
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
            Some(config_path) => match fs::read(&config_path) {
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
        Ok(data) => match toml::from_slice::<toml::Value>(&data) {
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
}
