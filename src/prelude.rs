use super::settings::*;
use std::{collections::HashMap, ops::Range};

type Dict<'a> = HashMap<u8, Vec<&'a str>>;

pub trait Builder: Default {
    fn with_words_count(&self, words_count: u8) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn with_word_lengths(&self, min_length: u8, max_length: u8) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn with_separators(&self, separators: &str) -> Self;
    fn with_padding_digits(&self, prefix: u8, suffix: u8) -> Self;
    fn with_padding_symbols(&self, symbols: &str) -> Self;
    fn with_padding_symbol_lengths(&self, prefix: u8, suffix: u8) -> Self;
    fn with_padding_strategy(
        &self,
        padding_strategy: PaddingStrategy,
    ) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn with_word_transforms(&self, transform: u8) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn from_preset(preset: Preset) -> Self;
}

pub trait Randomizer {
    fn word_lengths(&self) -> Range<u8>;
    fn rand_words(&self, pool: &[&str]) -> Vec<String>;
    fn rand_separator(&self) -> String;
    fn rand_prefix(&self) -> (String, String);
    fn rand_suffix(&self) -> (String, String);
    fn adjust_for_padding_strategy(&self, passwd: &str) -> String;
}

#[derive(Debug, Default)]
pub struct Xkpasswd {
    dict: Dict<'static>,
}

impl Xkpasswd {
    pub fn new() -> Xkpasswd {
        let dict_en_bytes = include_bytes!("./assets/dict_en.txt");
        let dict = load_dict(&dict_en_bytes[..]);
        Xkpasswd { dict }
    }

    pub fn gen_pass<S: Randomizer>(&self, settings: &S) -> String {
        let mut all_words: Vec<&str> = vec![];

        settings.word_lengths().for_each(|len| {
            if let Some(words) = self.dict.get(&len) {
                all_words.extend(words);
            };
        });

        let separator = &settings.rand_separator();
        let mut words: Vec<String> = vec![];

        let (prefix_symbols, prefix_digits) = settings.rand_prefix();
        if !prefix_digits.is_empty() {
            words.push(prefix_digits);
        }

        words.extend(settings.rand_words(&all_words));

        let (suffix_digits, suffix_symbols) = settings.rand_suffix();
        if !suffix_digits.is_empty() {
            words.push(suffix_digits);
        }

        let passwd = format!(
            "{}{}{}",
            prefix_symbols,
            words.join(separator),
            suffix_symbols
        );
        settings.adjust_for_padding_strategy(&passwd)
    }
}

fn load_dict(dict_bytes: &[u8]) -> Dict {
    let dict_str = std::str::from_utf8(dict_bytes).unwrap_or("");
    let mut dict: Dict = HashMap::new();

    dict_str.lines().for_each(|line| {
        let mut comps = line.split(':');

        if let Some(len_str) = comps.next() {
            let len = len_str.parse::<u8>().unwrap();
            let words_csv = comps.next().unwrap_or("");
            let words: Vec<&str> = words_csv.split(',').collect();
            dict.insert(len, words);
        }
    });

    dict
}
