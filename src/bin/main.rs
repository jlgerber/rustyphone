#[macro_use] extern crate prettytable;
use prettytable::{Table, format};
use sqlx::postgres::PgPoolOptions;
use structopt::StructOpt;
use std::collections::HashMap;

type RowMap = HashMap<Location, PhoneRow>;

// internal
use userdb::create;
use userdb::read;
use userdb::delete;

use userdb::DB_URL;
use userdb::Phone;
use userdb::PersonView;
use userdb::PhoneRow;
use userdb::read::person::PersonQuery;
use userdb::PhoneCategory;
use userdb::QueryMode;
use userdb::read::phone::PhoneQuery;
use userdb::Location;
use userdb::prelude::*;
use userdb::NumberString;

//-------------------------
// Structopt Structures
//-------------------------

#[derive(StructOpt, Debug)]
#[structopt(about="Search for people, and more...")]
struct Opt {

        /// Fetch phone records for people by full name
        #[structopt(short, long)]
        name: Option<String>,
    
        /// Fetch phone records for people by login
        #[structopt(short, long)]
        login: Option<String>,

        /// Fetch phone records for people by title
        #[structopt(short, long)]
        title: Option<String>,

        /// Fetch phone records for people by department
        #[structopt(short, long)]
        dept: Option<String>,

        /// Optionally specify the fullname (alternative to using --name flag)
        #[structopt(name="FULLNAME")]
        fullname: Option<String>,

        /// Display results as json instead of as a table
        #[structopt(short,long)]
        json: bool,

        /// Optional subcommands
        #[structopt(subcommand)]
        cmd: Option<OptSub>
}



/// Subcommands
#[derive(StructOpt, Debug)]
enum OptSub {

    /// Create entities
    Create {
        #[structopt(subcommand)]
        sub: CreateOpt,
    },

    /// Query entities (like title and department)
    Read {
        #[structopt(subcommand)]
        sub: ReadOpt,
    },

    Delete {
        #[structopt(subcommand)]
        sub: DeleteOpt,
    }
}

/// Create Subcommands
#[derive(StructOpt, Debug)]
enum CreateOpt {
    Person {

        /// Provide first name
        #[structopt(name = "FIRSTNAME")]
        first: String,

        /// Provide the last name
        #[structopt(name = "LASTNAME")]
        last: String,

        /// Provide the login
        #[structopt(name = "LOGIN")]
        login: String,

        /// Provide the department
        #[structopt(name = "DEPARTMENT")]
        department: String,

        /// Provide the title
        #[structopt(name = "TITLE")]
        title: String
    },
    Phone {

        /// specify the name of your login
        #[structopt(short="u", long)]
        login: String,

        /// Specify the number to match
        #[structopt(short, long)]
        number: NumberString,

        /// Specify the category of your phone number
        #[structopt(short, long)]
        category: PhoneCategory,

        /// Specify the location of your phone number
        #[structopt(short, long)]
        location: Location,

    }
}

/// Read Subcommands
#[derive(StructOpt, Debug)]
enum ReadOpt {
    /// Read contact information about people
    Person {
        /// Full name query
        #[structopt(short, long)]
        name: Option<String>,
    
        /// specify the name of your login
        #[structopt(short, long)]
        login: Option<String>,

        /// specify the title
        #[structopt(short, long)]
        title: Option<String>,

        /// specify the department
        #[structopt(short, long)]
        dept: Option<String>,

        /// Optionally specify the fullname (alternative to using --name flag)
        #[structopt(name="FULLNAME")]
        fullname: Option<String>,

        /// Display results as json instead of a table
        #[structopt(short,long)]
        json: bool,
    },
    Title {
        /// Display results as json instead of a table
        #[structopt(short,long)]
        json: bool,
    },
    Department {
        /// Display results as json instead of a table
        #[structopt(short,long)]
        json: bool,
    },
    Phone {
        /// specify the id
        #[structopt(short, long)]
        id: Option<u32>,

        /// Specify the number to match
        #[structopt(short, long)]
        number: Option<NumberString>,

        /// Specify the category of your phone number
        #[structopt(short, long)]
        category: Option<PhoneCategory>,

        /// Specify the location of your phone number
        #[structopt(short, long)]
        location: Option<Location>,

        /// Display results as json instead of as a table
        #[structopt(short,long)]
        json: bool,
    }
}
//
// DELETE
//
/// Delete Subcommands
#[derive(StructOpt, Debug)]
enum DeleteOpt {
    Phone {
        /// delete by phone id
        #[structopt(short,long)]
        id: Option<u32>,

        /// specify the name of your login
        #[structopt(
            short="u", long,
            requires_all = &["number", "category", "location"]
        )]
        login: Option<String>,

        /// Specify the number to match
        #[structopt(
            short, 
            long,
            requires_all = &["login", "category", "location"]
        )]
        number: Option<NumberString>,

        /// Specify the category of your phone number
        #[structopt(
            short, 
            long,
            requires_all = &["login", "number", "location"]
        )]
        category: Option<PhoneCategory>,

        /// Specify the location of your phone number
        #[structopt(
            short, 
            long,
            requires_all = &["login", "number", "category"]
        )]
        location: Option<Location>,
    }
}

//------------------------
// Async Command Handlers
//------------------------

//
// handle read person request
//
async fn process_read_person(
    name: Option<String>, 
    login: Option<String>, 
    title: Option<String>,
    dept: Option<String>,
    json: bool
) -> Result<(),sqlx::Error> {
    // verify that either name or login is set
    if name.is_none()  && 
       login.is_none() && 
       title.is_none() &&
       dept.is_none()
    {
        eprintln!("\nError: Must provide --name or --login or --title or --dept");
        std::process::exit(1);
    }

    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;

    // PersonQuery is just a simple ol' pod
    let personquery = PersonQuery::new()
                                .name(name)
                                .login(login)
                                .title(title)
                                .dept(dept);
    // query the database
    let results = read::person::query(&pool, personquery, QueryMode::ILike ).await?;

    // present the results - either in a table or as raw json, depending upon
    // whether the user has requested json via the --json flag or not
    if json {
        // convert to json and print it out. simple as can be
        let people = serde_json::to_string_pretty(&results).unwrap();
        println!("{}", people);
    } else {
        for result in results {
            let person: PersonView = serde_json::from_value(result).unwrap();
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_CLEAN);
            // we nest two tables for phones in order to achieve a better
            // aesthetic balance in formatting. To pull off this trick
            // prettytable allows us to embed one table within another.
            table.add_row(row![format!(" User: {}", person.login), format!(" Full Name: {}", person.fullname)]);
            table.add_row(row![format!(" Dept: {}", person.department), format!(" Title: {}", person.title)]);
            match person.phones {
                None => {
                    // Empty Row Handling
                    table.add_row(row![" Ext:      H:             ","P:       C:       Loc:       "]);
                },
                // currently, we assume that we will only have at most one of each type of phone per location
                Some(phones) => {
                    // rowmap maps location to phonerow
                    let mut rowmap = RowMap::new();
                    for phone in phones {
                        if !rowmap.contains_key(&phone.location) {
                            let mut phonerow = PhoneRow::new();
                            phonerow.location = Some(phone.location.clone());
                            rowmap.insert(phone.location.clone(), phonerow);
                        }
                        match phone.category {
                            PhoneCategory::Home =>      rowmap.get_mut(&phone.location).unwrap().home = Some(phone.number.clone()),
                            PhoneCategory::Extension => rowmap.get_mut(&phone.location).unwrap().ext  = Some(phone.number.clone()),
                            PhoneCategory::Cell =>      rowmap.get_mut(&phone.location).unwrap().cell = Some(phone.number.clone()),
                        }
                    }
                    // construct a new table per side. We are embedding the first table
                    // in the left column, and the second table in the right column
                    for (_loc, phonerow) in rowmap {
                        let mut table_l = Table::new();
                        let mut table_r = Table::new();
                        table_l.set_format(*format::consts::FORMAT_CLEAN);
                        table_r.set_format(*format::consts::FORMAT_CLEAN);
                        table_l.add_row(phonerow.row_left());
                        table_r.add_row(phonerow.row_right());
                        table.add_row(row![table_l.to_string(), table_r.to_string()]);
                    }
                }
            }
            table.printstd();
            println!();
        }
    }
    
    Ok(())
}

//
// handle read phone request
//
async fn process_read_phone(
    query: PhoneQuery,
    mode: QueryMode,
    json: bool,
) -> Result<(), sqlx::Error> {
    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;

    let results = read::phone::query(&pool, query, mode).await?;
    if json {
        let phones = serde_json::to_string_pretty(&results).unwrap();
        println!("{}", phones);
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.add_row(row![bFC->"ID", bFC->"NUMBER", bFC->"CATEGORY", bFC->"LOCATION"]);
        for result in results {
            let phone: Phone = serde_json::from_value(result).unwrap();

            table.add_row(row![
                phone.phone_id, 
                phone.number, 
                phone.category.to_static_str(), 
                phone.location.to_static_str()
            ]);
        }
        table.printstd();
    }
    Ok(())
}

//
// handle read title request
//
async fn process_read_title(json: bool)  -> Result<(), sqlx::Error> 
{
    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
    
    let results = read::title::query(&pool).await?;
    if json {
        let titles = serde_json::to_string_pretty(&results).unwrap();
        println!("{}", titles);
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.add_row(row![bFC->"NAME", bFC->"ID"]);
        for result in results {
            let title: read::title::TitleView = serde_json::from_value(result).unwrap();
            table.add_row(row![b->title.name, title.id]);
        }
        table.printstd();
    }
    Ok(())
}

//
// handle read  department request
//
async fn process_read_department(json: bool)  -> Result<(), sqlx::Error> 
{
    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
    
    let results = read::department::departmentview(&pool).await?;
    if json {
        let depts = serde_json::to_string_pretty(&results).unwrap();
        println!("{}", depts);
    } else {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.add_row(row![bFC->"NAME", bFC->"ID"]);
        for result in results {
            let dept: read::department::DepartmentView = serde_json::from_value(result).unwrap();
            table.add_row(row![b->dept.name, dept.id]);
        }
        table.printstd();
    }
    Ok(())
}


//
// handle create person request
//
async fn process_create_person(
    first: &str, 
    last:&str, 
    login: &str, 
    department: &str, 
    title: &str
) -> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = create::person::create(&pool, first, last, login, department, title).await?;
    println!("ID: {}", result);
    Ok(())
}

//
// handle create phone request
//
async fn process_create_phone(
    login: &str, 
    number:&NumberString, 
    category: &PhoneCategory, 
    location: &Location, 
    
) -> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = create::phone::create(&pool, login, number, category, location).await?;
    match result {
        Some(val) => println!("Created Phone with id: {}", val),
        None => println!("Phone number already exists")
    };
    Ok(())
}

//
// handle delete  record between an individual and a phone request
//
async fn process_delete_phone(
    login: &str, 
    number:&NumberString, 
    category: &PhoneCategory, 
    location: &Location, 
    
) -> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::person_phone::delete(&pool, login, number, category, location).await?;
    match result {
        Some(val) => println!("Deleted Phone with id: {}", val),
        None => println!("Phone number not associated with {}", login)
    };
    Ok(())
}

//
// handle delete a phone by id request
//
async fn process_delete_phone_by_id(id: u32) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::phone::delete_by_id(&pool, id).await?;
    match result {
        Some(val) => println!("Deleted Phone with id: {}", val),
        None => println!("Id {} does not exist", id)
    };
    Ok(())
}


#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // build options from structopt
    let opt = Opt::from_args();
    match opt {
        Opt{mut name, login, json, title, dept, fullname, cmd: None} => {
            if name.is_none() && fullname.is_some() {
                name = fullname;
            }
            process_read_person(name, login, title, dept, json ).await},
        Opt{cmd: Some(OptSub::Read{sub}), ..} => match sub {
            ReadOpt::Person{mut name, login, title, dept, fullname, json} => {
                if name.is_none() && fullname.is_some() {
                    name = fullname;
                }
                process_read_person(name, login, title, dept, json).await
            },
            ReadOpt::Title{json} => process_read_title(json).await,
            ReadOpt::Department{json} => process_read_department(json).await,
            ReadOpt::Phone{id, number, category, location, json} => {
                
                let query = PhoneQuery::new()
                .id(id)
                .number(number)
                .category(category)
                .location(location);
                process_read_phone(query, QueryMode::ILike, json ).await}
        }
        Opt{cmd: Some(OptSub::Create{sub}), ..} => match sub {
            CreateOpt::Person{first, last, login, department, title} => process_create_person(&first, &last, &login, &department, &title).await,
            CreateOpt::Phone{login, number, category, location} => process_create_phone(&login, &number, &category, &location).await,
        }
        Opt{cmd: Some(OptSub::Delete{sub}), ..} => match sub {
            DeleteOpt::Phone{id: Some(id),..} => process_delete_phone_by_id(id).await,
            DeleteOpt::Phone{
                login: Some(login), 
                number: Some(number), 
                category:Some(category), 
                location: Some(location),..} => process_delete_phone(&login, &number, &category, &location).await,
                _ => panic!("Cannot reach here do to requires_all runtime constraints")
        }
    }
}
