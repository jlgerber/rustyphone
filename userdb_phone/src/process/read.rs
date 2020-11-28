use colored::*;
use prettytable::{Table, format, row, cell};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;

type RowMap = HashMap<Location, PhoneRow>;

// internal
use userdb_core::read;

use userdb_core::prelude::*;
use userdb_core::DB_URL;
use userdb_core::Phone;
use userdb_core::PersonView;
use userdb_core::PhoneRow;
use userdb_core::read::person::PersonQuery;
use userdb_core::read::person_simple::SimplePersonQuery;
use userdb_core::PhoneCategory;
use userdb_core::QueryMode;
use userdb_core::read::phone::PhoneQuery;
use userdb_core::Location;


/// Process a request to read data from the database and present it to the user 
/// via stdout/stderr, as either a table (default) or json (if `json` is true)
pub async fn process_read_simple_person(
    value: String, 
    json: bool
) -> Result<(),sqlx::Error> {
   

    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;

    // PersonQuery is just a simple ol' pod
    let personquery = SimplePersonQuery::new(value);
    // query the database
    let results = read::person_simple::query(&pool, personquery, QueryMode::ILike ).await?;

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
            table.add_row(row![format!(" {} {}", "User:".bright_cyan(), person.login), format!(" {} {}", "Full Name:".bright_cyan(), person.fullname)]);
            table.add_row(row![format!(" {} {}","Dept:".bright_cyan(),  person.department), format!(" {} {}","Title:".bright_cyan(), person.title)]);
            match person.phones {
                None => {
                    // Empty Row Handling
                    table.add_row(row![" Ext:      H:             ".bright_cyan(),"P:       C:       Loc:       ".bright_cyan()]);
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

/// Process a request to read data from the database and present it to the user 
/// via stdout/stderr, as either a table (default) or json (if `json` is true)
pub async fn process_read_person(
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
        eprintln!("\n{}: Must provide --name or --login or --title or --dept", "Error".bright_red());
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
            table.add_row(row![format!(" {} {}", "User:".bright_cyan(), person.login), format!(" {} {}", "Full Name:".bright_cyan(), person.fullname)]);
            table.add_row(row![format!(" {} {}","Dept:".bright_cyan(),  person.department), format!(" {} {}","Title:".bright_cyan(), person.title)]);
            match person.phones {
                None => {
                    // Empty Row Handling
                    table.add_row(row![" Ext:      H:             ".bright_cyan(),"P:       C:       Loc:       ".bright_cyan()]);
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

/// Process the request to read phone data from the database, and print it to stderr/stdout
/// in either a table (default) or json
pub async fn process_read_phone(
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

/// Process the request to read titles from the database and print them to 
/// stderr/stdout as a table (default) or json.
pub async fn process_read_title(json: bool)  -> Result<(), sqlx::Error> 
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

/// Process the request to print departments to stderr/stdout as a table (default)
/// or json (via the `json` argument)
pub async fn process_read_department(json: bool)  -> Result<(), sqlx::Error> 
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
