#[cfg(test)]
mod tests;

#[cfg(test)]
mod proptest_tests;

use std::collections::HashMap;
use std::fmt;
use std::ops::Range;
use std::str::*;
use wasm_bindgen::prelude::*;

use crate::error::XkpasswdError;

/// Represents the padding strategy for password generation.
///
/// Padding strategies determine how symbols are added to passwords to reach
/// desired lengths or provide additional complexity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaddingStrategy {
    /// Fixed-length padding - symbols are added as specified in settings
    Fixed,
    /// Adaptive padding - adjusts total password length to the specified value
    Adaptive(usize),
}

/// Result of applying padding to a password.
///
/// This enum represents the three possible outcomes when padding is applied
/// to a password during generation.
#[derive(Debug, Clone)]
pub enum PaddingResult {
    /// Password remains unchanged, no padding was needed
    Unchanged,
    /// Password should be trimmed to the specified length
    TrimTo(usize),
    /// Password should have the specified padding string appended
    Pad(String),
}

/// Predefined password generation presets.
///
/// Each preset provides a complete configuration optimized for specific use cases,
/// with settings for word count, length, separators, and padding appropriate for
/// different security contexts.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Preset {
    /// Default balanced settings suitable for most use cases
    Default,
    /// Optimized for Apple ID password requirements
    AppleID,
    /// Compatible with Windows NTLM v1 password constraints  
    WindowsNtlmV1,
    /// Designed for security question answers that need to be memorable
    SecurityQuestions,
    /// 16-character web-friendly passwords
    Web16,
    /// 32-character web-friendly passwords  
    Web32,
    /// WiFi network passwords with high entropy
    Wifi,
    /// XKCD-style passwords (4 words with simple separators)
    Xkcd,
}

/// Represents the estimated time required to crack a password through brute force.
///
/// This structure breaks down the cracking time into years, months, and days,
/// making it easier to understand and display password strength to users.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GuessTime {
    /// Number of years required for brute force attack
    pub years: usize,
    /// Additional months beyond the years
    pub months: u8,
    /// Additional days beyond years and months
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
    /// Number of password guesses attempted per second in brute force attacks
    pub const GUESSES_PER_SEC: usize = 1_000;
    const SECONDS_PER_DAY: f64 = 86_400.0;
    const DAYS_PER_MONTH: f64 = 30.0;
    const DAYS_PER_YEAR: f64 = 365.0;

    /// Calculate guess time required for given entropy amount.
    ///
    /// # Arguments
    ///
    /// * `amount` - The entropy amount in bits
    ///
    /// # Returns
    ///
    /// A `GuessTime` struct representing the estimated crack time
    #[must_use]
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

/// Represents the entropy (randomness) measurements of a generated password.
///
/// Entropy is measured in bits and determines how difficult a password is to crack.
/// This structure provides both blind entropy (assuming no knowledge of the generation
/// method) and seen entropy (assuming full knowledge of the generation method).
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Entropy {
    /// Minimum blind entropy in bits (conservative estimate)
    pub blind_min: usize,
    /// Maximum blind entropy in bits (optimistic estimate)
    pub blind_max: usize,
    /// Entropy in bits when generation method is fully known
    pub seen: usize,
    /// Estimated time to crack the password
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

/// Supported languages for word dictionaries.
///
/// Each language provides a curated dictionary of words suitable for
/// password generation, filtered for appropriateness and memorability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    /// English language dictionary
    English,
    /// French language dictionary  
    French,
    /// German language dictionary
    German,
    /// Portuguese language dictionary
    Portuguese,
    /// Spanish language dictionary
    Spanish,
}

type Dict<'a> = HashMap<u8, Vec<&'a str>>;

/// Trait for localization support.
///
/// Types implementing this trait can be created with language-specific
/// dictionaries and resources.
pub trait L10n {
    /// Create an instance configured for the specified language.
    ///
    /// # Arguments
    ///
    /// * `language` - The target language for localization
    ///
    /// # Returns
    ///
    /// A new instance configured with the appropriate language resources
    fn for_language(language: Language) -> Self;
}

/// Trait for building password generation settings using a fluent interface.
///
/// This trait provides a builder pattern for configuring password generation
/// parameters. All methods return modified instances, allowing for method chaining.
pub trait Builder: Default + fmt::Display + Sized {
    /// Set the number of words in the generated password.
    ///
    /// # Arguments
    ///
    /// * `words_count` - Number of words to include (must be positive)
    ///
    /// # Errors
    ///
    /// Returns `XkpasswdError::InvalidWordsCount` if words_count is 0
    fn with_words_count(&self, words_count: u8) -> Result<Self, XkpasswdError>;

    /// Configure the allowed word length range.
    ///
    /// # Arguments  
    ///
    /// * `min_length` - Minimum word length (must be at least 4 if specified)
    /// * `max_length` - Maximum word length (must be at most 10 if specified)
    ///
    /// # Errors
    ///
    /// Returns `XkpasswdError::MinWordLengthTooSmall` or `XkpasswdError::MaxWordLengthTooLarge`
    /// for invalid length constraints
    fn with_word_lengths(
        &self,
        min_length: Option<u8>,
        max_length: Option<u8>,
    ) -> Result<Self, XkpasswdError>;

    /// Set the characters to use as word separators.
    ///
    /// # Arguments
    ///
    /// * `separators` - String containing possible separator characters
    #[must_use]
    fn with_separators(&self, separators: &str) -> Self;

    /// Configure padding with random digits.
    ///
    /// # Arguments
    ///
    /// * `prefix` - Number of digits to add at the beginning (None for no prefix)
    /// * `suffix` - Number of digits to add at the end (None for no suffix)
    fn with_padding_digits(&self, prefix: Option<u8>, suffix: Option<u8>) -> Self;

    /// Set the symbols to use for padding.
    ///
    /// # Arguments
    ///
    /// * `symbols` - String containing possible padding symbols
    fn with_padding_symbols(&self, symbols: &str) -> Self;

    /// Configure padding symbol lengths.
    ///
    /// # Arguments
    ///
    /// * `prefix` - Number of symbols to add at the beginning (None for no prefix)
    /// * `suffix` - Number of symbols to add at the end (None for no suffix)  
    fn with_padding_symbol_lengths(&self, prefix: Option<u8>, suffix: Option<u8>) -> Self;

    /// Set the padding strategy.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The padding strategy to use
    ///
    /// # Errors
    ///
    /// Returns `XkpasswdError::InvalidAdaptivePadding` for invalid adaptive lengths
    fn with_padding_strategy(&self, strategy: PaddingStrategy) -> Result<Self, XkpasswdError>;

    /// Configure word transformations.
    ///
    /// # Arguments
    ///
    /// * `transform` - Bitflags representing the transformations to apply
    ///
    /// # Errors
    ///
    /// Returns `XkpasswdError::InvalidTransform` for invalid transform combinations
    fn with_word_transforms(&self, transform: u8) -> Result<Self, XkpasswdError>;

    /// Create a new instance from a preset configuration.
    ///
    /// # Arguments
    ///
    /// * `preset` - The preset configuration to use
    fn from_preset(preset: Preset) -> Self;
}

/// Trait for password generation randomization logic.
///
/// This trait encapsulates the random selection and transformation logic
/// used during password generation, including word selection, separator
/// choice, padding application, and entropy calculation.
pub trait Randomizer {
    /// Get the range of allowed word lengths.
    ///
    /// # Returns
    ///
    /// A range representing minimum and maximum word lengths
    fn word_lengths(&self) -> Range<u8>;

    /// Select and transform random words from the word pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - Slice of available words to choose from
    ///
    /// # Returns
    ///
    /// Vector of transformed words ready for use in password generation
    fn rand_words(&self, pool: &[&str]) -> Vec<String>;

    /// Select a random separator character.
    ///
    /// # Returns
    ///
    /// A string containing the selected separator
    fn rand_separator(&self) -> String;

    /// Generate random prefix padding (symbols and digits).
    ///
    /// # Returns
    ///
    /// A tuple of (symbols, digits) for the password prefix
    fn rand_prefix(&self) -> (String, String);

    /// Generate random suffix padding (digits and symbols).
    ///
    /// # Returns
    ///
    /// A tuple of (digits, symbols) for the password suffix
    fn rand_suffix(&self) -> (String, String);

    /// Adjust password padding based on the current length.
    ///
    /// # Arguments
    ///
    /// * `pass_length` - Current password length
    ///
    /// # Returns
    ///
    /// A `PaddingResult` indicating how to adjust the password
    fn adjust_padding(&self, pass_length: usize) -> PaddingResult;

    /// Calculate entropy for the given word pool size.
    ///
    /// # Arguments
    ///
    /// * `pool_size` - Size of the available word pool
    ///
    /// # Returns
    ///
    /// An `Entropy` struct containing calculated entropy values
    fn calc_entropy(&self, pool_size: usize) -> Entropy;
}

/// Main password generator with language-specific word dictionaries.
///
/// `Xkpasswd` combines a word dictionary with password generation settings
/// to create memorable, secure passwords using the XKCD approach of combining
/// random words with separators and padding.
#[derive(Debug)]
pub struct Xkpasswd {
    dict: Dict<'static>,
}

impl Default for Xkpasswd {
    fn default() -> Self {
        if cfg!(feature = "lang_en") {
            Xkpasswd::for_language(Language::English)
        } else if cfg!(feature = "lang_de") {
            Xkpasswd::for_language(Language::German)
        } else if cfg!(feature = "lang_es") {
            Xkpasswd::for_language(Language::Spanish)
        } else if cfg!(feature = "lang_fr") {
            Xkpasswd::for_language(Language::French)
        } else if cfg!(feature = "lang_pt") {
            Xkpasswd::for_language(Language::Portuguese)
        } else {
            panic!("no language bundled")
        }
    }
}

impl L10n for Xkpasswd {
    fn for_language(language: Language) -> Self {
        let dict_bytes: &[u8] = match language {
            #[cfg(feature = "lang_en")]
            Language::English => include_bytes!("../assets/dict_en.txt"),
            #[cfg(feature = "lang_de")]
            Language::German => include_bytes!("../assets/dict_de.txt"),
            #[cfg(feature = "lang_es")]
            Language::Spanish => include_bytes!("../assets/dict_es.txt"),
            #[cfg(feature = "lang_fr")]
            Language::French => include_bytes!("../assets/dict_fr.txt"),
            #[cfg(feature = "lang_pt")]
            Language::Portuguese => include_bytes!("../assets/dict_pt.txt"),
            #[allow(unreachable_patterns)]
            _ => panic!("no language bundled"),
        };

        let dict = load_dict(dict_bytes);
        Xkpasswd { dict }
    }
}

impl Xkpasswd {
    /// Generate a password using the specified settings.
    ///
    /// This method combines words from the loaded dictionary with separators,
    /// digits, and symbols according to the provided settings to create a
    /// memorable yet secure password.
    ///
    /// # Arguments
    ///
    /// * `settings` - Configuration implementing the `Randomizer` trait
    ///
    /// # Returns
    ///
    /// A tuple containing the generated password string and its entropy information
    #[must_use]
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

fn load_dict(dict_bytes: &[u8]) -> Dict<'_> {
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
