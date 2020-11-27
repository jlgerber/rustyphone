
use colored::*;
use sqlx::postgres::PgPoolOptions;
//use std::collections::HashMap;

// internal
use userdb_core::update;
use userdb_core::DB_URL;
//use userdb_core::PhoneRow;
use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::prelude::*;
use userdb_core::NumberString;
use userdb_core::update::person::id::PersonUpdate as PersonUpdateById;
use userdb_core::update::person::login::PersonUpdate as PersonUpdateByLogin;
use userdb_core::update::phone::id::PhoneUpdate as PhoneUpdateById;
use userdb_core::update::title::id::TitleUpdate as TitleUpdateById;
use userdb_core::update::department::id::DepartmentUpdate as DepartmentUpdateById;

//type RowMap = HashMap<Location, PhoneRow>;


pub async fn process_update_person_by_id(
    id: i32, 
    first: Option<String>, 
    last: Option<String>, 
    login: Option<String>, 
    department: Option<i32>, 
    title: Option<i32>) -> Result<(), sqlx::Error> 
    {
        let person_update = PersonUpdateById::new(id)
        .first(first)
        .last(last)
        .login(login)
        .department(department)
        .title(title);

        if person_update.is_empty() {
            eprintln!("\n\t{} Nothing to do updating person. No changes supplied", "Warning:".bright_green());
            return Ok(());
        }

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(DB_URL).await?;

        let result = update::person::id::update(&pool, person_update).await;
        let result = match result {
            Ok(r) => r,
            Err(sqlx::Error::RowNotFound) =>  None,
            Err(e) => return Err(e)
        };
        match result {
            Some(val) => println!("Updated person with id: {}", val),
            None => eprintln!("\n\tNothing to do updating person"),
        };
        Ok(())
    }

    
pub async fn process_update_person_by_login(
    from_login: String, 
    first: Option<String>, 
    last: Option<String>, 
    login: Option<String>, 
    department: Option<i32>, 
    title: Option<i32>) 
-> Result<(), sqlx::Error> {
    let person_update = PersonUpdateByLogin::new(from_login)
    .first(first)
    .last(last)
    .login(login)
    .department(department)
    .title(title);
    
    if person_update.is_empty() {
        eprintln!("\n\t{} Nothing to do updating person. No changes supplied", "Warning:".bright_green());
        return Ok(());
    }

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;

    let result = update::person::login::update(&pool, person_update).await;
    let result = match result {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) =>  None,
        Err(e) => return Err(e)
    };
    match result {
        Some(val) => println!("Updated person with id: {}", val),
        None => eprintln!("\n\tNothing to do updating person"),
    };
    Ok(())
}

pub async fn process_update_phone_by_id(
    // the phone's id
    id: i32, 
    number: Option<NumberString>, 
    category: Option<PhoneCategory>, 
    location: Option<Location>, 
) -> Result<(), sqlx::Error> 
{
    let phone_update = PhoneUpdateById::new(id)
                        .number(number)
                        .category(category)
                        .location(location);

    if phone_update.is_empty() {
        eprintln!("\n\t{} Nothing to do updating phone. No changes supplied", "Warning:".bright_green());
        return Ok(());
    }

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;

    let result = update::phone::id::update(&pool, phone_update).await;
    let result = match result {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) =>  None,
        Err(e) => return Err(e)
    };
    match result {
        Some(val) => println!("Updated phone with id: {}", val),
        None => eprintln!("\n\tNothing to do updating phone"),
    };
    Ok(())
}


pub async fn process_update_title_by_id(
    // the phone's id
    id: i32, 
    name: String, 
) -> Result<(), sqlx::Error> 
{
    let title_update = TitleUpdateById::new(id, name);
   
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;

    let result = update::title::id::update(&pool, title_update).await;
    let result = match result {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) =>  None,
        Err(e) => return Err(e)
    };
    match result {
        Some(val) => println!("Updated title with id: {}", val),
        None => eprintln!("\n\tNothing to do updating title"),
    };
    Ok(())
}


pub async fn process_update_dept_by_id(
    // the phone's id
    id: i32, 
    name: String, 
) -> Result<(), sqlx::Error> 
{
    let department_update = DepartmentUpdateById::new(id, name);
   
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;

    let result = update::department::id::update(&pool, department_update).await;
    let result = match result {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) =>  None,
        Err(e) => return Err(e)
    };
    match result {
        Some(val) => println!("Updated department with id: {}", val),
        None => eprintln!("\n\tNothing to do updating department"),
    };
    Ok(())
}