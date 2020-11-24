use async_std;
#[macro_use] extern crate prettytable;
use prettytable::{Table, format};
use sqlx::postgres::PgPoolOptions;
use structopt::StructOpt;

// internal
use userdb::create;
use userdb::DB_URL;
use userdb::PersonView;
use userdb::PhoneRow;
use userdb::read;
use userdb::read::person::PersonQuery;

//-------------------------
// Structopt Structures
//-------------------------
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
        /// Specify the number to match
        #[structopt(short, long)]
        number: Option<String>,

        /// Specify the category of your phone number
        #[structopt(short, long)]
        category: Option<String>,

        /// Specify the location of your phone number
        #[structopt(short, long)]
        location: Option<String>,
    }

}

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
        cmd: Option<Opt2>
}

#[derive(StructOpt, Debug)]
enum Opt2 {

    /// Create entities
    Create {

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

    /// Query entities (like title and department)
    Read {
        #[structopt(subcommand)]
        sub: ReadOpt,
    }
}


//------------------------
// Async Command Handlers
//------------------------

//
// process reading of person
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
        .max_connections(5)
        .connect(DB_URL).await?;

    // PersonQuery is just a simple ol' pod
    let personquery = PersonQuery::new()
                                .name(name)
                                .login(login)
                                .title(title)
                                .dept(dept);
    // query the database
    let results = read::person::query(&pool, personquery, read::person::QueryMode::ILike ).await?;

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
            // prettytable allows us to mbed one table within another.
            let mut table_r1 = Table::new();
            let mut table_r2 = Table::new();
            table_r1.set_format(*format::consts::FORMAT_CLEAN);
            table_r2.set_format(*format::consts::FORMAT_CLEAN);
    
            table.add_row(row![format!(" User: {}",person.login), format!("Full Name: {}",person.fullname)]);
            table.add_row(row![format!(" Dept: {}", person.department), format!("Title: {}", person.title)]);
            match person.phones {
                None => {
                    // Empty Row Handling
                    table.add_row(row![" Ext:     H:       ","P:       C:       Loc:       "]);
                },
                // currently, we rely on having at most a single entry for each category. 
                // this can be upgraded with a bit of work, by keeping track of what has already
                // been created. Each location will have a vector of phonerow instances
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
    }
    
    Ok(())
}

//
// process the read phone command
//
async fn process_read_phone(
    number: Option<String>, 
    category: Option<String>, 
    location: Option<String>
) -> Result<(), sqlx::Error> {
    println!("{:?} {:?} {:?}", number, category, location);
    Ok(())
}

//
// Process read title request
//
async fn process_read_title(json: bool)  -> Result<(), sqlx::Error> 
{
    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await?;
    
    let results = read::title::titleview(&pool).await?;
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
// process read department request
//
async fn process_read_department(json: bool)  -> Result<(), sqlx::Error> 
{
    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(5)
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
// process creation of person
//
async fn process_create(
    first: &str, 
    last:&str, 
    login: &str, 
    department: &str, 
    title: &str
) -> Result<(),sqlx::Error> {
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
        Opt{mut name, login, json, title, dept, fullname, cmd: None} => {
            if name.is_none() && fullname.is_some() {
                name = fullname;
            }
            process_read_person(name, login, title, dept, json ).await},
        Opt{cmd: Some(Opt2::Read{sub}), ..} => match sub {
            ReadOpt::Person{mut name, login, title, dept, fullname, json} => {
                if name.is_none() && fullname.is_some() {
                    name = fullname;
                }
                process_read_person(name, login, title, dept, json).await
            },
            ReadOpt::Title{json} => process_read_title(json).await,
            ReadOpt::Department{json} => process_read_department(json).await,
            ReadOpt::Phone{number, category, location} => process_read_phone(number, category, location).await
        }
        Opt{cmd: Some(Opt2::Create{first, last, login, department, title}), ..} => process_create(&first, &last, &login, &department, &title).await,
    }
}
