use prettytable::{Row, Cell};
//------------------
// PhoneRow struct
//------------------

#[derive(Debug)]
pub struct PhoneRow {
    pub ext: Option<String>,
    pub home: Option<String>,
    pub pager: Option<String>,
    pub cell: Option<String>,
    pub location: Option<String>
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


fn format_phone(num: &Option<String>, label: &str) -> Cell {
    if num.is_some() {
        
        let num = num.as_deref().unwrap();
        let ccnt = num.chars().count();
        if ccnt == 7 {
            // assuming ascii numbers
            Cell::new(&format!("{}: {}-{}-{}",label,&num[..3], &num[3..6], &num[6..] ))
        } else {
            Cell::new(&format!("{}: {}", label, num))
        }
    } else {
        Cell::new(&format!("{}:     ", label))
    }
}