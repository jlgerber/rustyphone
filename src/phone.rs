// external crates
use serde::{Serialize, Deserialize, de};
use sqlx::FromRow;
use std::str::FromStr;

// internal crate
use crate::Location;
use crate::PhoneCategory;
use crate::PhoneNumber;

/// Struct which represents a Phone entry in the 
/// backing database. It is also capable of serializing
/// and deserializing via serde
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Phone {
    pub phone_id: i32,
    #[serde(deserialize_with = "deserialize_phonenumber")]
    pub number: PhoneNumber,
    pub category: PhoneCategory,
    pub location: Location
} 

// custom deserializer for PhoneNumber. Because the different variants
// of PhoneNumber are indistinguishable based on shape (ie they all just wrap String), 
// specifying the tag as internal does not work. So, we write a little custom deal.
fn deserialize_phonenumber<'de, D>(deserializer: D) -> Result<PhoneNumber, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    let num = PhoneNumber::from_str(&s).map_err(de::Error::custom)?;
    Ok(num)
}