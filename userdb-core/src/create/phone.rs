//! C in crud
//! create

//use crate::PersonView;
use std::convert::AsRef;
use sqlx::prelude::*;
use crate::prelude::*;

const CREATE_PHONE: &str = r"
SELECT 
    *
FROM
    addPhone($1, $2, $3::phonecategory, $4::location);
";

#[derive(FromRow)]
struct Rval {
    addphone: Option<i32>
}

pub async fn create<I>(
    pool: &sqlx::PgPool, 
    login: I, 
    number: &crate::NumberString, 
    category: &crate::PhoneCategory, 
    location: &crate::Location
) -> Result<Option<i32>, sqlx::Error>
where
    I: AsRef<str>,
{
    let number = number.to_string();
    let Rval{addphone} = sqlx::query_as(&CREATE_PHONE)
    .bind(login.as_ref())
    .bind(number)
    .bind(category.to_static_str())
    .bind(location.to_static_str())
    .fetch_one(pool).await?;
    //let Rval{addphone} = Rval::from_row(&row).unwrap();
    Ok(addphone)
}