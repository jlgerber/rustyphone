pub trait Selectable {
    /// Return the select comma separated list of columns 
    /// as a String 
    fn select(&self) -> String;
}