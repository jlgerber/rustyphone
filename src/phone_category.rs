
use std::str::FromStr;
use crate::PhoneError;
use serde::{Serialize,Deserialize};
use crate::traits::ToStaticStr;
use std::fmt;

/// The category of a phone number - either a `Home` number, a `Cell`
/// phone number, or an internal `Extension`.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename="phonecategory")]//, rename_all = "lowercase")]
pub enum PhoneCategory {
    Home,
    Cell,
    Extension
}

impl FromStr for PhoneCategory {
    type Err = PhoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "home" | "h" => Ok(Self::Home),
            "cell" | "c" => Ok(Self::Cell),
            "ext" | "extension" | "e" => Ok(Self::Extension),
            _ => Err(Self::Err::InvalidCategory(s.to_string()))
        }
    }
}

impl ToStaticStr for PhoneCategory {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Cell => "Cell",
            Self::Extension => "Extension"
        }
    }
}

impl fmt::Display for PhoneCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::PhoneErrorKind;

    #[test]
    fn from_str_given_valid_strs_constructs_category() {
        let tests = &[
            (&vec!["home", "h"], PhoneCategory::Home),
            (&vec!["cell", "c"], PhoneCategory::Cell),
            (&vec!["ext", "extension", "e"], PhoneCategory::Extension)
        ];

        for test in tests {
            for item in test.0 {
                let result = PhoneCategory::from_str(item).unwrap();
                assert_eq!(result, test.1);
            }
        }
    }

    #[test]
    fn from_str_given_bad_data_constructs_err() {
        let result = PhoneCategory::from_str("foobar").unwrap_err().kind();
        assert_eq!(result, PhoneErrorKind::InvalidCategory);
    }

    #[test]
    fn to_static_str_returns_str() {
        let tests = &[
            (PhoneCategory::Home, "Home"),
            (PhoneCategory::Cell, "Cell"),
            (PhoneCategory::Extension, "Extension")
        ];

        for test in tests {
            let result = test.0.to_static_str();
            assert_eq!(result, test.1);
        }
    }

    #[test]
    fn to_string_returns_str() {
        let tests = &[
            (PhoneCategory::Home, "Home"),
            (PhoneCategory::Cell, "Cell"),
            (PhoneCategory::Extension, "Extension")
        ];

        for test in tests {
            let result = test.0.to_string();
            assert_eq!(result.as_str(), test.1);
        }
    }

}