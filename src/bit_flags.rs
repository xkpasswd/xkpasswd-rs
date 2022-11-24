use std::{fmt, ops::*};
use wasm_bindgen::prelude::*;

pub type FieldSize = u8;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordTransform {
    // single transforms - possible to combine with each other
    Lowercase = 0b00000001,
    Titlecase = 0b00000010,
    Uppercase = 0b00000100,
    InversedTitlecase = 0b00001000,

    // group transforms - overriding other single ones
    AltercaseLowerFirst = 0b01000000,
    AltercaseUpperFirst = 0b10000000,
}

impl fmt::Display for WordTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::Lowercase => "lowercase",
            Self::Uppercase => "UPPERCASE",
            Self::Titlecase => "Titlecase",
            Self::InversedTitlecase => "iNVERSED tITLECASE",
            Self::AltercaseLowerFirst => "altercase LOWER first",
            Self::AltercaseUpperFirst => "ALTERCASE upper FIRST",
        };

        write!(f, "{}", name)
    }
}

pub trait BitFlags {
    fn from_flag(flag: WordTransform) -> Self;
    fn has_flag(self, flag: WordTransform) -> bool;
    fn to_flags(self) -> Vec<WordTransform>;
    fn to_strings(self) -> Vec<String>;
}

impl BitOr for WordTransform {
    type Output = FieldSize;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as FieldSize | rhs
    }
}

impl BitOr<WordTransform> for FieldSize {
    type Output = FieldSize;

    fn bitor(self, rhs: WordTransform) -> Self::Output {
        self | rhs as FieldSize
    }
}

impl BitAnd<WordTransform> for FieldSize {
    type Output = bool;

    fn bitand(self, rhs: WordTransform) -> Self::Output {
        self & rhs as FieldSize > 0
    }
}

impl BitFlags for FieldSize {
    fn from_flag(flag: WordTransform) -> Self {
        flag as Self
    }

    fn has_flag(self, flag: WordTransform) -> bool {
        self & flag
    }

    fn to_flags(self) -> Vec<WordTransform> {
        let mut flags: Vec<WordTransform> = vec![];
        for flag in [
            WordTransform::AltercaseUpperFirst,
            WordTransform::AltercaseLowerFirst,
            WordTransform::Lowercase,
            WordTransform::Titlecase,
            WordTransform::Uppercase,
            WordTransform::InversedTitlecase,
        ] {
            if self & flag {
                flags.push(flag)
            }
        }

        flags
    }

    fn to_strings(self) -> Vec<String> {
        WordTransform::to_strings(&self.to_flags())
    }
}

impl WordTransform {
    pub fn to_strings(transforms: &[WordTransform]) -> Vec<String> {
        transforms
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_or() {
        assert_eq!(
            0b00000101,
            WordTransform::Lowercase | WordTransform::Uppercase
        );
        assert_eq!(
            0b00001010,
            WordTransform::Titlecase | WordTransform::InversedTitlecase
        );
        assert_eq!(
            0b10000001,
            WordTransform::Lowercase | WordTransform::AltercaseUpperFirst
        );
        assert_eq!(
            0b01001010,
            WordTransform::Titlecase
                | WordTransform::InversedTitlecase
                | WordTransform::AltercaseLowerFirst
        );
    }

    #[test]
    fn test_from_flag() {
        assert_eq!(0b00000001, FieldSize::from_flag(WordTransform::Lowercase));
        assert_eq!(0b00000010, FieldSize::from_flag(WordTransform::Titlecase));
        assert_eq!(0b00000100, FieldSize::from_flag(WordTransform::Uppercase));
        assert_eq!(
            0b00001000,
            FieldSize::from_flag(WordTransform::InversedTitlecase)
        );
        assert_eq!(
            0b01000000,
            FieldSize::from_flag(WordTransform::AltercaseLowerFirst)
        );
        assert_eq!(
            0b10000000,
            FieldSize::from_flag(WordTransform::AltercaseUpperFirst)
        );
    }

    #[test]
    fn test_has_flag() {
        let transforms = WordTransform::Lowercase
            | WordTransform::Uppercase
            | WordTransform::AltercaseLowerFirst;
        assert!(transforms.has_flag(WordTransform::Lowercase));
        assert!(transforms.has_flag(WordTransform::Uppercase));
        assert!(transforms.has_flag(WordTransform::AltercaseLowerFirst));
        assert!(!transforms.has_flag(WordTransform::Titlecase));
        assert!(!transforms.has_flag(WordTransform::InversedTitlecase));
        assert!(!transforms.has_flag(WordTransform::AltercaseUpperFirst));
    }

    #[test]
    fn test_to_flags() {
        let transforms = WordTransform::Titlecase | WordTransform::InversedTitlecase;
        assert_eq!(
            vec![WordTransform::Titlecase, WordTransform::InversedTitlecase,],
            transforms.to_flags()
        );

        let transforms = WordTransform::Lowercase
            | WordTransform::Uppercase
            | WordTransform::AltercaseLowerFirst;
        assert_eq!(
            vec![
                WordTransform::AltercaseLowerFirst,
                WordTransform::Lowercase,
                WordTransform::Uppercase,
            ],
            transforms.to_flags()
        );
    }
}
