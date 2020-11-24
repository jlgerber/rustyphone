/// A Selectable implements `select`, which simply returns a 
/// comma separated list of columns.
pub trait Selectable {
    /// Return the select comma separated list of columns 
    /// as a String 
    fn select(&self) -> String;
}