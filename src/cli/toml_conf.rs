use super::*;
use crate::bit_flags::*;
use clap::ValueEnum;
use std::fs;

const CONFIG_FILE_NAME: &str = "xkpasswd.toml";

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

    if let Some(field_value) = config.get(field) {
        if let Some(value_str) = field_value.as_str() {
            match T::from_str(value_str, true) {
                Ok(value) => {
                    callback(value);
                    log::debug!("loading '{}' from config file", field);
                    Ok(())
                }
                Err(err) => Err(ConfigParseError::InvalidConfig(field.to_string(), err)),
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
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

    if let Some(value) = config.get(field) {
        if let Some(transforms) = value.as_array() {
            let result: Result<Vec<&str>, String> = transforms
                .iter()
                .map(|transform| {
                    transform.as_str().ok_or(format!(
                        "Invalid data type, expect string but got '{}'",
                        transform
                    ))
                })
                .collect();

            let raw_transforms = match result {
                Err(err) => {
                    return Err(ConfigParseError::InvalidConfig(
                        "word_transforms".to_string(),
                        err,
                    ))
                }
                Ok(transforms) => transforms,
            };

            let parsed_transforms: Result<Vec<WordTransform>, String> = raw_transforms
                .iter()
                .map(|transform| WordTransform::from_str(transform, true))
                .collect();

            match parsed_transforms {
                Ok(word_transforms) => {
                    callback(word_transforms);
                    log::debug!("loading '{}' from config file", field);
                }
                Err(err) => {
                    return Err(ConfigParseError::InvalidConfig(
                        "word_transforms".to_string(),
                        err,
                    ))
                }
            };
        }
    }
    Ok(())
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

    if let Some(field_value) = config.get(field) {
        if let Some(value) = field_value.as_str() {
            callback(value.to_string());
            log::debug!("loading '{}' from config file", field);
        }
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

    match get_number(config, field) {
        Some(value) => {
            callback(value);
            log::debug!("loading '{}' from config file", field);
        }
        None => log::debug!("loading default value for '{}'", field),
    }
}

fn get_number(config: &toml::Value, field: &str) -> Option<u64> {
    config
        .get(field)?
        .as_integer()
        .map(|data| data.unsigned_abs())
}
