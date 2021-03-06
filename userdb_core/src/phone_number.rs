use serde::{Serialize, Deserialize};
use std::str::FromStr;
use crate::PhoneError;
use std::fmt;

/// Representation of a phone number
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum PhoneNumber {
    /// An extension consists of four numbers
    Extension(String),
    /// An area code prefix'ed number works for the US an Canada
    TenDigit(String),
    /// All other forms
    Other(String),
}

impl PhoneNumber {
    /// Return a copy of the inner value as a String.
    pub fn inner(&self) -> String {
        match self {
            Self::Extension(ext) => ext.clone(),
            Self::TenDigit(tdig) => tdig.clone(),
            Self::Other(other) => other.clone()
        }
    }
}

impl FromStr for PhoneNumber {
    type Err = PhoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let characters = s.chars().collect::<Vec<_>>();
        if  !characters.iter().all(|x| x.is_numeric()) {
            return Err(PhoneError::InvalidNumber(s.to_string()))
        }

        match s {
            _ if characters.len() == 4  => Ok( Self::Extension( s.to_string() ) ),
            _ if characters.len() == 10 => Ok( Self::TenDigit( s.to_string() ) ),
            _ if characters.len() < 7   => Err( Self::Err::InvalidNumber( s.to_string() ) ),
            _ => Ok(Self::Other(s.to_string())),
        }
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TenDigit(tdig) => write!(f, "{}-{}-{}", &tdig[0..3], &tdig[3..6], &tdig[6..]),
            Self::Extension(ext) => write!(f, "{}", ext),
            Self::Other(other)   => write!(f, "{}", other)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PhoneErrorKind;

    #[test]
    fn from_str_given_valid_strs_succeeds() {
        let tests = &[
            ("3103864321", PhoneNumber::TenDigit("3103864321".into())),
            ("4321", PhoneNumber::Extension("4321".into())),
            ("32345678", PhoneNumber::Other("32345678".into()))
        ];
        for (num, expect) in tests {
            let results = PhoneNumber::from_str(num).unwrap();
            assert_eq!(&results, expect);
        }
    }

    #[test]
    fn from_str_given_bad_input_should_fail() {
        let inputs = &["afb123", "123a4567a", "123456", "12345", "123", "12", "1"];
        for input in inputs {
            let result = PhoneNumber::from_str(input).unwrap_err().kind();
            assert_eq!(result, PhoneErrorKind::InvalidNumber)
        }
    }

    #[test]
    fn to_string_given_valid_strs_succeeds() {
        let tests = &[
            ( PhoneNumber::TenDigit( "3103864321".into() ), "310-386-4321".to_string() ),
            ( PhoneNumber::Extension( "4321".into() ),"4321".into() ),
            ( PhoneNumber::Other( "32345".into() ),"32345".into() )
        ];
        for (num, expect) in tests {
            let results = num.to_string();
            assert_eq!(&results, expect);
        }
    }

    #[test]
    fn inner_returns_copy_of_inner_string() {
        let tests = &[
            ( PhoneNumber::TenDigit("3103864321".into()), "3103864321".to_string()),
            ( PhoneNumber::Extension("4321".into()), "4321".into()),
            ( PhoneNumber::Other("32345".into()), "32345".into())
        ];
        for (num, expect) in tests {
            let results = num.inner();
            assert_eq!(&results, expect);
        }
    }

}