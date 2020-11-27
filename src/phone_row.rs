use prettytable::{Row, Cell};
use crate::PhoneNumber;
use crate::Location;
use colored::*;


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
       
        row.push(format_phone(ext, "Ext: ",4));
        row.push(format_phone(home, "H:",12));        
        Row::new(row)
    }
    /// return the right row. 
    pub fn row_right(&self) -> Row {
        let Self{pager, cell, location,..} = self;
        let mut row = Vec::new();
        row.push(format_phone(pager, "P:",4));
        row.push(format_phone(cell, "C:",12));
        match location {
            Some(loc) => row.push(Cell::new(&format!("{} {}","Loc:".bright_blue(), loc))),
            None => row.push(Cell::new(&format!("{}", "Loc:".bright_blue())))
        };
        Row::new(row)
    }
}


fn format_phone(num: &Option<PhoneNumber>, label: &str, width: u8) -> Cell {
    if num.is_some() {
        let num = num.as_ref().unwrap();
        Cell::new(&format!("{} {}", label.bright_blue(), num))
    } else {
        Cell::new(&format!("{} {:width$}", label.bright_blue(), " ", width=width as usize))
    }
}