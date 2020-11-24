use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhoneError {
    #[error("invalid category supplied '{0}'. Should be one of: home, cell, ext")]
    InvalidCategory(String)
}