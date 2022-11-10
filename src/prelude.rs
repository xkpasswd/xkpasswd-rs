use super::settings::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::collections::HashMap;

pub type Dict<'a> = HashMap<u8, Vec<&'a str>>;

pub fn gen_passwd(dict: &Dict, settings: &Settings) -> String {
    let mut all_words: Vec<&str> = vec![];

    let (min, max) = settings.word_lengths;
    for len in min..(max + 1) {
        if let Some(words) = dict.get(&len) {
            all_words.extend(words);
        }
    }

    let mut rng = rand::thread_rng();
    let word_indices = Uniform::from(0..all_words.len());

    let words = (0..settings.words_count)
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
        .collect::<Vec<String>>()
        .join(".");

    let suffix = {
        let padding_digits: u8 = Uniform::from(10..100).sample(&mut rng);
        let padding_symbols: Vec<char> = PADDING_SYMBOLS.chars().collect();
        let padding_symbol = padding_symbols[rng.gen_range(0..PADDING_SYMBOLS.len())];

        format!("{}{}{}", padding_digits, padding_symbol, padding_symbol)
    };

    format!("{}.{}", words, suffix)
}

pub fn load_dict(dict_bytes: &[u8]) -> Dict {
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

#[cfg(feature = "benchmarks")]
#[cfg(test)]
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
