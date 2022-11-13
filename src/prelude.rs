use super::settings::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::collections::HashMap;

type Dict<'a> = HashMap<u8, Vec<&'a str>>;

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

    pub fn gen_pass(&self, settings: &Settings) -> String {
        gen_passwd(&self.dict, settings)
    }
}

fn gen_passwd(dict: &Dict, settings: &Settings) -> String {
    let mut all_words: Vec<&str> = vec![];

    for len in settings.word_lengths() {
        if let Some(words) = dict.get(&len) {
            all_words.extend(words);
        }
    }

    let mut rng = rand::thread_rng();
    let word_indices = Uniform::from(0..all_words.len());
    let separator = &settings.rand_separator().to_string();

    let words: Vec<String> = (0..settings.words_count())
        .map(|_| loop {
            let index: usize = word_indices.sample(&mut rng);
            let word = all_words[index];

            if !word.is_empty() {
                all_words[index] = "";

                let display_word = if rng.gen::<bool>() {
                    word.to_uppercase()
                } else {
                    word.to_string()
                };

                break display_word;
            }
        })
        .collect();

    let rand_prefix = settings.rand_prefix();
    let prefix = if rand_prefix.is_empty() {
        rand_prefix
    } else {
        format!("{}{}", rand_prefix, separator)
    };

    let rand_suffix = settings.rand_suffix();
    let suffix = if rand_suffix.is_empty() {
        rand_suffix
    } else {
        format!("{}{}", separator, rand_suffix)
    };

    let symbols: Vec<char> = DEFAULT_SYMBOLS.chars().collect();
    let rand_symbol = symbols[rng.gen_range(0..DEFAULT_SYMBOLS.len())];
    let padding_symbols = format!("{}{}", rand_symbol, rand_symbol);

    format!(
        "{}{}{}{}",
        prefix,
        words.join(separator),
        suffix,
        padding_symbols
    )
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

#[cfg(all(feature = "benchmarks", test))]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_load_dict(b: &mut Bencher) {
        b.iter(|| {
            let dict_en_bytes = include_bytes!("./assets/dict_en.txt");
            load_dict(&dict_en_bytes[..]);
        })
    }

    #[bench]
    fn bench_xkpasswd(b: &mut Bencher) {
        let dict_en_bytes = include_bytes!("./assets/dict_en.txt");
        let dict_en = &load_dict(&dict_en_bytes[..]);
        let settings = &Settings::default().words_count(3).word_lengths(3, 8);
        b.iter(|| gen_passwd(dict_en, settings));
    }
}
