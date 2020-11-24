use std::str::FromStr;
use crate::traits::ToStaticStr;
use crate::errors::PhoneError;
use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Debug,PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename="location", rename_all = "lowercase")]
pub enum Location {
    Portland,
    PlayaVista,
    Vancouver,
    Montreal,
    Hyderabad
}


impl FromStr for Location {
    type Err = PhoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "portland" | "pd"  => Ok(Self::Portland),
            "playa" | "playavista" | "playa vista" | "venice" | "pv"=> Ok(Self::PlayaVista),
            "vancouver" | "bc"  => Ok(Self::Vancouver),
            "montreal" | "mt" => Ok(Self::Montreal),
            "hyderabald" | "hb" => Ok(Self::Hyderabad),
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