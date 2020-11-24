
use std::str::FromStr;
use crate::PhoneError;
use serde::{Serialize,Deserialize};
use crate::traits::ToStaticStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename="phonecategory", rename_all = "lowercase")]
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