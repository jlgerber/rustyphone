use async_std;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};
use sqlx::postgres::PgPoolOptions;
use structopt::StructOpt;
// internal
use userdb::read;
use userdb::create;
use userdb::DB_URL;
use userdb::QueryParam;
use userdb::PersonView;


#[derive(StructOpt, Debug)]
#[structopt(about="crud operations for phone command")]
enum Opt {
    /// create a person
    Create {

        ///provide first name
        #[structopt(name = "FIRSTNAME")]
        first: String,
        /// provide the last name
        #[structopt(name = "LASTNAME")]
        last: String,
        /// provide the login
        #[structopt(name = "LOGIN")]
        login: String,
        /// provide teh department
        #[structopt(name = "DEPARTMENT")]
        department: String,
        /// provide the title
        #[structopt(name = "TITLE")]
        title: String
    },
    /// query the userdb for a person or persons matching the 
    /// supplied argument
    Read {
        /// Full name query
        #[structopt(short, long)]
        name: Option<String>,
    
        /// specify the name of your login
        #[structopt(short, long)]
        login: Option<String>.
    }
}

#[derive(Debug)]
struct PhoneRow {
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
       // row.push(format_phone(pager, "P"));
        
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

async fn process_read(name: Option<String>, login: Option<String>) -> Result<(),sqlx::Error> {
    // verify that either name or login is set
    if name.is_none() && login.is_none() {
        eprintln!("\nError: Must provide --name or --login");
        std::process::exit(1);
    }
    if name.is_some() && login.is_some() {
        eprintln!("\nError: must select either login or name");
        std::process::exit(1);
    }
    
    // build out the query param, assuming that if name is set,
    // then login is not set
    let query_param = if name.is_some() {
        //QueryParam::ilike_name(name.unwrap())
        QueryParam::new(name.unwrap(), read::QueryField::Name, read::QueryMode::ILike)
    } else {
        //QueryParam::ilike_login(login.unwrap())
        QueryParam::new(login.unwrap(), read::QueryField::Login, read::QueryMode::ILike)

    };
    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await?;
        
    let results = read::personview(&pool,query_param).await?;
    for result in results {
        let person: PersonView = serde_json::from_value(result).unwrap();
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        let mut table_r1 = Table::new();
        let mut table_r2 = Table::new();
        table_r1.set_format(*format::consts::FORMAT_CLEAN);
        table_r2.set_format(*format::consts::FORMAT_CLEAN);

        table.add_row(row![format!(" User: {}",person.login), format!("Full Name: {}",person.fullname)]);
        table.add_row(row![format!(" Dept: {}", person.department), format!("Title: {}", person.title)]);
        match person.phones {
            None => {
                table.add_row(row![" Ext:     H:       ","P:       C:       Loc:       "]);
            },
            Some(phones) => {
                let mut phonerow = PhoneRow::new();
                for phone in phones {
                    match phone.category.as_ref() {
                        "home" => phonerow.home = Some(phone.number.clone()),
                        "extension" | "ext" => phonerow.ext = Some(phone.number.clone()),
                        "cell" => phonerow.cell = Some(phone.number.clone()),
                        _ => ()
                    }
                    phonerow.location = Some(phone.location.clone());
                }
                table_r1.add_row(phonerow.row_left());
                table_r2.add_row(phonerow.row_right());
                table.add_row(row![table_r1.to_string(), table_r2.to_string()]);
            }
        }
        table.printstd();
        println!("");
       
    }
    Ok(())
}

async fn process_create(first: &str, last:&str, login: &str, department: &str, title: &str) 
-> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await?;
        
    let result = create::person(&pool, first, last, login, department, title).await?;
    println!("ID: {}", result);
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // build options from structopt
    let opt = Opt::from_args();
    match opt {
        Opt::Read{name, login} => process_read(name, login).await,
        Opt::Create{first, last, login, department, title} 
            => process_create(&first, &last, &login, &department, &title).await
    }
}
