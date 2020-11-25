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