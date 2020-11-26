use std::str::FromStr;
use crate::traits::ToStaticStr;
use crate::PhoneError;
use std::fmt;
use serde::{Serialize, Deserialize};

/// An enumerated list of DD locations, matching the enum in the database.
#[derive(Debug,PartialEq, Eq, Serialize, Deserialize, Clone, sqlx::Type, Hash)]
#[sqlx(rename="location")] //, rename_all = "lowercase")]
pub enum Location {
    Portland,
    PlayaVista,
    Vancouver,
    Montreal,
    Hyderabad
}


impl FromStr for Location {
    type Err = PhoneError;
    /// Convert from a str to a Location. The input is 
    /// case insensitive, and accepts both the name and 
    /// two letter designation for the location. 
    ///
    /// In the case of Playa, we accept a number of different
    /// aliases, including with a variety of separators between
    /// words `playa` and `vista`, including a dash, a space, and
    /// nothing. We also accept the legacy `venice` disgnation.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "portland" | "pd"  => Ok(Self::Portland),
            "playa" | "playavista" | "playa vista" | "playa-vista"| "venice" | "pv"=> Ok(Self::PlayaVista),
            "vancouver" | "bc"  => Ok(Self::Vancouver),
            "montreal" | "mt" => Ok(Self::Montreal),
            "hyderabad" | "hb" => Ok(Self::Hyderabad),
            _ => Err(Self::Err::InvalidLocation(s.to_string()))
        }
    }
}

impl ToStaticStr for Location {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::Portland => "Portland",
            Self::PlayaVista => "PlayaVista",
            Self::Vancouver => "Vancouver",
            Self::Montreal => "Montreal",
            Self::Hyderabad => "Hyderabad"
        }
        
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}