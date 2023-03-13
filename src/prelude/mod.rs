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
    Adaptive(usize),
}

#[derive(Debug)]
pub enum PaddingResult {
    Unchanged,
    TrimTo(usize),
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

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GuessTime {
    pub years: usize,
    pub months: u8,
    pub days: u8,
}

impl fmt::Display for GuessTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.years > 1_000_000_000 {
            return write!(f, "more than a billion years");
        }

        if self.years > 1_000_000 {
            return write!(f, "more than a million years");
        }

        if self.years > 1_000 {
            return write!(f, "more than a thousand years");
        }

        let mut comps: Vec<String> = vec![];

        if self.years > 0 {
            comps.push(format!("{} years", self.years))
        }

        if self.months > 0 {
            comps.push(format!("{} months", self.months))
        }

        if self.days > 0 {
            comps.push(format!("{} days", self.days))
        }

        if comps.is_empty() {
            write!(f, "less than a day")
        } else {
            write!(f, "{}", comps.join(" "))
        }
    }
}

impl GuessTime {
    pub const GUESSES_PER_SEC: usize = 1_000;
    const SECONDS_PER_DAY: f64 = 86_400.0;
    const DAYS_PER_MONTH: f64 = 30.0;
    const DAYS_PER_YEAR: f64 = 365.0;

    pub fn for_entropy(amount: usize) -> Self {
        if amount > 64 {
            return Self {
                years: 1_000_000_001,
                months: 0,
                days: 0,
            };
        }

        if amount > 54 {
            return Self {
                years: 1_000_001,
                months: 0,
                days: 0,
            };
        }

        if amount > 44 {
            return Self {
                years: 1001,
                months: 0,
                days: 0,
            };
        }

        let mut time_to_guess =
            2.0f64.powi(amount as i32) / (Self::SECONDS_PER_DAY * Self::GUESSES_PER_SEC as f64);

        let years = (time_to_guess / Self::DAYS_PER_YEAR).floor() as usize;
        time_to_guess -= Self::DAYS_PER_YEAR * years as f64;

        let months = (time_to_guess / Self::DAYS_PER_MONTH).floor() as u8;
        time_to_guess -= Self::DAYS_PER_MONTH * months as f64;

        let days = time_to_guess.floor() as u8;

        Self {
            days,
            months,
            years,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Entropy {
    pub blind_min: usize,
    pub blind_max: usize,
    pub seen: usize,
    pub guess_time: GuessTime,
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
            "{} blind and {} bits with full knowledge, which takes computers {} to break at {} guesses/sec",
            blind_entropies, self.seen, self.guess_time, GuessTime::GUESSES_PER_SEC
        )
    }
}

type Dict<'a> = HashMap<u8, Vec<&'a str>>;

pub trait Builder: Default + fmt::Display + Sized {
    fn with_words_count(&self, words_count: u8) -> Result<Self, String>;
    fn with_word_lengths(
        &self,
        min_length: Option<u8>,
        max_length: Option<u8>,
    ) -> Result<Self, String>;
    fn with_separators(&self, separators: &str) -> Self;
    fn with_padding_digits(&self, prefix: Option<u8>, suffix: Option<u8>) -> Self;
    fn with_padding_symbols(&self, symbols: &str) -> Self;
    fn with_padding_symbol_lengths(&self, prefix: Option<u8>, suffix: Option<u8>) -> Self;
    fn with_padding_strategy(&self, strategy: PaddingStrategy) -> Result<Self, String>;
    fn with_word_transforms(&self, transform: u8) -> Result<Self, String>;
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
            PaddingResult::TrimTo(len) => passwd[..len].to_string(),
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
