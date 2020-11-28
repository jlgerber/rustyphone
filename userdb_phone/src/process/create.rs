
use colored::*;
use sqlx::postgres::PgPoolOptions;

// internal
use userdb_core::create;
use userdb_core::DB_URL;
use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::NumberString;


//
// handle create person request
//

/// Process the request to create a person, given required fields
pub async fn process_create_person(
    first: &str, 
    last: &str, 
    login: &str, 
    department: &str, 
    title: &str
) -> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = create::person::create(&pool, first, last, login, department, title).await?;
    println!("{} {}","ID:".bright_green(), result);
    Ok(())
}

/// Process request to create a phone entry for an individual, specified via the login
pub async fn process_create_phone(
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
        None => eprintln!("\n\t{} Phone number already exists", "Warning:".bright_green())
    };
    Ok(())
}

/// Process the request to create a new title
pub async fn process_create_title(
    title: &str,     
) -> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = create::title::create(&pool, &title).await;
    let result = match result {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) =>  None,
        Err(e) => return Err(e)
    };
    match result {
        Some(val) => println!("Created Title with id: {}", val),
        None => eprintln!("\n\t{} Title '{}' already exists", "Warning:".bright_cyan(), title)
    };
    Ok(())
}

/// Process the request to create a new department
pub async fn process_create_department(
    department: &str,     
) -> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL).await?;
        
    let result = create::department::create(&pool, &department).await;
    let result = match result {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) =>  None,
        Err(e) => return Err(e)
    };
    match result {
        Some(val) => println!("Created Department with id: {}", val),
        None => eprintln!("\n\t{} Department '{}' already exists", "Warning:".bright_green(), department)
    };
    Ok(())
}
