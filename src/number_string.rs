#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;
use crate::PhoneError;

/// A wrapper around a string that ensures all of the characters are integers
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct NumberString {
    inner: String
}

impl FromStr for NumberString {
    type Err = PhoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(|x| x.is_numeric()) {
            return Err(Self::Err::InvalidNumber(s.to_string()))
        }

        Ok(Self{inner: s.to_string()})
    }
}

impl fmt::Display for NumberString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl NumberString {
    /// Construct a new NumberString
    pub fn new(value: &str) -> Result<Self, PhoneError> {
        Self::from_str(value)
    }

    /// construct a NumberString from a usize
    pub fn from_usize(value: usize) -> Self {
       Self::from_str(&value.to_string()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PhoneErrorKind;

    #[test]
    fn from_str_converts_if_presented_solely_with_digits() {
        let results = NumberString::from_str("12345").unwrap();
        assert_eq!(results, NumberString{inner: "12345".to_string()})
    }

    #[test]
    fn from_str_fails_if_presented_with_nondigits() {
        let results = NumberString::from_str("12345a").unwrap_err().kind();
        assert_eq!(results, PhoneErrorKind::InvalidNumber);
    }
    #[test]
    fn from_usize_constructs_NumberString() {
        let results = NumberString::from_usize(12345);
        assert_eq!(results, NumberString{inner: "12345".to_string()})
    }
}