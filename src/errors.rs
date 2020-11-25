use thiserror::Error;
use enum_kinds::EnumKind;

/// Custom error enum for the phone project.
#[derive(Error, Debug, EnumKind)]
#[enum_kind(PhoneErrorKind)]
pub enum PhoneError {
    #[error("Invalid category supplied '{0}'. Should be one of: home, cell, ext")]
    InvalidCategory(String),

    #[error("Invalid location supplied '{0}'")]
    InvalidLocation(String),

    #[error("Invalid number supplied '{0}'")]
    InvalidNumber(String)
}

impl PhoneError {
    /// Construct a PhoneErrorKind from a PhoneError. The PhoneErrorKind
    /// is a simpler enum with no data. It can be used to test for equality,
    /// Whereas PhoneError does not derive PartialEq or Eq 
    pub fn kind(&self) -> PhoneErrorKind {
        PhoneErrorKind::from(self)
    }
}