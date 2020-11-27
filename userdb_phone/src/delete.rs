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

//
// handle delete  record between an individual and a phone request
//
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

//
// handle delete a phone by id request
//
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


