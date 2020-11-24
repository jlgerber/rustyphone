use crate::QueryMode;

/// A Selectable implements `select`, which simply returns a 
/// comma separated list of columns.
pub trait Selectable {
    /// Return the select comma separated list of columns 
    /// as a String 
    fn select(&self) -> String;
}

pub trait Queryable {
    fn query(&self, mode: &QueryMode) -> String ;
}
/// Convert a type to a static str
pub trait ToStaticStr {
    fn to_static_str(&self) -> &'static str;
}