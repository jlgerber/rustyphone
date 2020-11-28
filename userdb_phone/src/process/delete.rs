use sqlx::postgres::PgPoolOptions;
//use std::collections::HashMap;
use colored::*;

//type RowMap = HashMap<Location, PhoneRow>;

// internal
use userdb_core::delete;
use userdb_core::DB_URL;
//use userdb_core::PhoneRow;
use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::NumberString;

/// Process the request to delete a phone, identified by a user's login along iwth 
/// the number, category, and location. This function will do the following things:
/// - Remove association between the phone matching the `number`, `category`, and `location`, 
///   and the user, identified by `login`.
/// - Delete the phone record, as long as no additional releationships exist between the phone
///   and another user. (Users may share extentions for instance)
pub async fn process_delete_phone(
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
        Some(val) => println!("Deleted Phone?: {}", val>0),
        None => eprintln!("\n\t{} Phone number not associated with {}", "Warning:".bright_green(), login)
    };
    Ok(())
}

/// Process a request to delete a phone entry provided its `id` in the phone table.
pub async fn process_delete_phone_by_id(id: u32) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::phone::delete_by_id(&pool, id).await?;
    match result {
        Some(val) => println!("Deleted Phone with id: {}", val),
        None => eprintln!("\n\t{} Id {} does not exist","Warning:".bright_green(), id)
    };
    Ok(())
}

/// Process the request to delete a department by name. This method will not 
/// delete the department, should it be associated with one or more `person`s.
pub async fn process_delete_dept(name: &str) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::department::delete(&pool, name).await?;
    match result {
        Some(val) => println!("Deleted Dept with id: {}", val),
        None => eprintln!("\n\t{} Dept '{}' does not exist","Warning:".bright_cyan(), name)
    };
    Ok(())
}

/// Process request to delete a department given its `id` in the `department` table. This
/// function will not attempt to delete the department if the department is associated with
/// one or more `person`s.
pub async fn process_delete_dept_by_id(id: u32) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::department::delete_by_id(&pool, id).await?;
    match result {
        Some(val) => println!("Deleted Department with id: {}", val),
        None => eprintln!("\n\t{} Department Id '{}' does not exist", "Warning:".bright_green(), id)
    };
    Ok(())
}

/// Process request to delete the supplied title. This will not succeed if the 
/// `title` is in use by one or more `person`s.
pub async fn process_delete_title(name: &str) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::title::delete(&pool, name).await?;
    match result {
        Some(val) => println!("Deleted Title with id: {}", val),
        None => eprintln!("\n\t{} Title '{}' does not exist", "Warning:".bright_cyan(),name)
    };
    Ok(())
}

/// Process the request to delete a `title` by its `id` in the `title` table. This 
/// will not succeed if the `title` is associated with one or more `person`s.
pub async fn process_delete_title_by_id(id: u32) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::title::delete_by_id(&pool, id).await?;
    match result {
        Some(val) => println!("Deleted Department with id: {}", val),
        None => eprintln!("\n\t{} Title Id '{}' does not exist","Warning:".bright_green(), id)
    };
    Ok(())
}

/// Process request to delete a `person` by `login`.
pub async fn process_delete_person(login: &str) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::person::delete(&pool, login).await?;
    match result {
        Some(val) => println!("Deleted Person with id: {}", val),
        None => eprintln!("\n\t{} Person '{}' does not exist", "Warning:".bright_green(), login)
    };
    Ok(())
}

/// Process the request to delete a `person` by their `id` in the `person` table. 
pub async fn process_delete_person_by_id(id: u32) -> Result<(), sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = delete::person::delete_by_id(&pool, id).await?;
    match result {
        Some(val) => println!("Deleted Person with id: {}", val),
        None => eprintln!("\n\tPerson Id '{}' does not exist", id)
    };
    Ok(())
}


