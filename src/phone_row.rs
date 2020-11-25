use prettytable::{Row, Cell};
use crate::PhoneNumber;
use crate::Location;


/// Represents a presentation row of phone data as a struct 
/// with fields whose native types are all wrapped in Options.
/// This struct is used to transform the data returned from
/// the database into a reasonable line of information in
/// the terminal
#[derive(Debug)]
pub struct PhoneRow {
    pub ext: Option<PhoneNumber>,
    pub home: Option<PhoneNumber>,
    pub pager: Option<PhoneNumber>,
    pub cell: Option<PhoneNumber>,
    pub location: Option<Location>
}

impl Default for PhoneRow {
    fn default() -> Self {
        Self {
            ext: None,
            home:None,
            pager: None,
            cell: None,
            location: None
        }
    }
}

impl PhoneRow {
    pub fn new() -> Self {
        Self::default()
    }
    /// return the left row.
    pub fn row_left(&self) -> Row {
        let Self{ext, home,..} = self;
        let mut row = Vec::new();
       
        row.push(format_phone(ext, "Ext"));
        row.push(format_phone(home, "H"));        
        Row::new(row)
    }
    /// return the right row. 
    pub fn row_right(&self) -> Row {
        let Self{pager, cell, location,..} = self;
        let mut row = Vec::new();
        row.push(format_phone(pager, "P"));
        row.push(format_phone(cell, "C"));
        match location {
            Some(loc) => row.push(Cell::new(&format!("Loc: {}", loc))),
            None => row.push(Cell::new("Loc:"))
        };
        Row::new(row)
    }
}


fn format_phone(num: &Option<PhoneNumber>, label: &str) -> Cell {
    if num.is_some() {
        let num = num.as_ref().unwrap();
        Cell::new(&format!("{}: {}", label, num))
    } else {
        Cell::new(&format!("{}:     ", label))
    }
}