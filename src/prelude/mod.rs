#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
use std::str::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaddingStrategy {
    Fixed,
    Adaptive(u8),
}

#[derive(Debug)]
pub enum PaddingResult {
    Unchanged,
    TrimTo(u8),
    Pad(String),
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Preset {
    Default,
    AppleID,
    WindowsNtlmV1,
    SecurityQuestions,
    Web16,
    Web32,
    Wifi,
    Xkcd,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Entropy {
    pub blind_min: usize,
    pub blind_max: usize,
    pub seen: usize,
}

impl fmt::Display for Entropy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let blind_entropies = if self.blind_min == self.blind_max {
            format!("{} bits", self.blind_min)
        } else {
            format!("between {} & {} bits", self.blind_min, self.blind_max)
        };

        write!(
            f,
            "{} blind and {} bits with full knowledge",
            blind_entropies, self.seen
        )
    }
}

type Dict<'a> = HashMap<u8, Vec<&'a str>>;

pub trait Builder: Default + fmt::Display + Sized {
    fn with_words_count(&self, words_count: u8) -> Result<Self, &'static str>;
    fn with_word_lengths(
        &self,
        min_length: Option<u8>,
        max_length: Option<u8>,
    ) -> Result<Self, &'static str>;
    fn with_separators(&self, separators: &str) -> Self;
    fn with_padding_digits(&self, prefix: Option<u8>, suffix: Option<u8>) -> Self;
    fn with_padding_symbols(&self, symbols: &str) -> Self;
    fn with_padding_symbol_lengths(&self, prefix: Option<u8>, suffix: Option<u8>) -> Self;
    fn with_padding_strategy(&self, strategy: PaddingStrategy) -> Result<Self, &'static str>;
    fn with_word_transforms(&self, transform: u8) -> Result<Self, &'static str>;
    fn from_preset(preset: Preset) -> Self;
}

pub trait Randomizer {
    fn word_lengths(&self) -> Range<u8>;
    fn rand_words(&self, pool: &[&str]) -> Vec<String>;
    fn rand_separator(&self) -> String;
    fn rand_prefix(&self) -> (String, String);
    fn rand_suffix(&self) -> (String, String);
    fn adjust_padding(&self, pass_length: usize) -> PaddingResult;
    fn calc_entropy(&self, pool_size: usize) -> Entropy;
}

#[derive(Debug)]
pub struct Xkpasswd {
    dict: Dict<'static>,
}

impl Default for Xkpasswd {
    fn default() -> Self {
        let dict_en_bytes = include_bytes!("../assets/dict_en.txt");
        let dict = load_dict(&dict_en_bytes[..]);
        Xkpasswd { dict }
    }
}

impl Xkpasswd {
    pub fn gen_pass<S: Randomizer>(&self, settings: &S) -> (String, Entropy) {
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

        let passwd = match settings.adjust_padding(passwd.len()) {
            PaddingResult::Unchanged => passwd,
            PaddingResult::TrimTo(len) => passwd[..len as usize].to_string(),
            PaddingResult::Pad(padded_symbols) => passwd + &padded_symbols,
        };

        let entropy = settings.calc_entropy(all_words.len());

        (passwd, entropy)
    }
}

fn load_dict(dict_bytes: &[u8]) -> Dict {
    let dict_str = from_utf8(dict_bytes).unwrap_or("").trim();
    let mut dict: Dict = HashMap::new();

    log::debug!("loaded raw dict with {} lines", dict_str.lines().count());

    dict_str.lines().for_each(|line| {
        let mut comps = line.trim().split(':');

        if let Some(len_str) = comps.next() {
            let len = len_str.parse::<u8>().unwrap();
            let words_csv = comps.next().unwrap_or("");
            let words: Vec<&str> = words_csv.split(',').collect();
            dict.insert(len, words);
        }
    });

    log::debug!(
        "parsed dict with {:?} entries",
        dict.iter().fold(0, |acc, cur| acc + cur.1.len())
    );

    dict
}
