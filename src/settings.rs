use serde::{Deserialize, Serialize};

pub const PADDING_SYMBOLS: &str = "!@#$%^&*-_=+:|~?/;";

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub words_count: u8,
    pub word_lengths: (u8, u8),
}

impl Settings {
    pub fn words_count(&self, words_count: u8) -> Settings {
        Settings {
            words_count,
            ..(*self)
        }
    }

    pub fn word_lengths(&self, min_length: u8, max_length: u8) -> Settings {
        let word_lengths = if min_length > max_length {
            (max_length, min_length)
        } else {
            (min_length, max_length)
        };

        Settings {
            word_lengths,
            ..(*self)
        }
    }
}
