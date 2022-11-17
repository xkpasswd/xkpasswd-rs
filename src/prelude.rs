use super::settings::*;
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

    pub fn gen_pass<S: Randomizer>(&self, settings: &S) -> String {
        let mut all_words: Vec<&str> = vec![];

        settings.iter_word_lengths(|len| {
            if let Some(words) = self.dict.get(&len) {
                all_words.extend(words);
            };
        });

        let separator = &settings.rand_separator();
        let words = settings.rand_words(&all_words).join(separator);

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

        let passwd = format!("{}{}{}", prefix, words, suffix);
        settings.adjust_for_padding_strategy(passwd)
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
