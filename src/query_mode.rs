use std::fmt;

/// The query mode identified how the receiver should
/// treat the requested query. 
/// - ILike tests to see if the supplied param is a substring of 
///   the target value, ignoring case.
/// - Like works like `ILike` but pays attention to case
/// - Exact matches exactly
#[derive(Debug, PartialEq, Eq)]
pub enum QueryMode {
    ILike,
    Like,
    Exact
}
impl QueryMode {
    /// Return the comparsion operator as a static str 
    pub fn comparison(&self) -> &'static str {
        match *self {
            Self::ILike => "ILIKE",
            Self::Like => "LIKE",
            Self::Exact => "=",
        }
    }
}
impl fmt::Display for QueryMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ILike => write!(f, "ILIKE"),
            Self::Like => write!(f, "LIKE"),
            Self::Exact => write!(f, "="),
        }
    }
}