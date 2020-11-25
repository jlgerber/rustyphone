//! D in crud - Delete phone
use std::convert::AsRef;
use sqlx::prelude::*;
use crate::prelude::*;

const DELETE_PHONE: &str = r"
WITH phone_cte AS (
    SELECT 
        phone_id 
    FROM 
        personview
    WHERE
        login = $1
    AND 
        number = $2
    AND
        category = $3::phonecategory
    AND
        location = $4::location

)
DELETE FROM 
    phone 
WHERE
    phone.id = (
        SELECT 
            phone_id 
        FROM 
            phone_cte
    )
RETURNING id
";

const DELETE_PHONE_ID: &str = r"

DELETE FROM 
    phone 
WHERE
    phone.id = $1
RETURNING id
";

#[derive(FromRow)]
struct Rval {
    id: Option<i32>
}

/// Delete a phone that matches the provided parameters
pub async fn delete<I>(
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
    let Rval{id} = sqlx::query_as(&DELETE_PHONE)
    .bind(login.as_ref())
    .bind(number)
    .bind(category.to_static_str())
    .bind(location.to_static_str())
    .fetch_one(pool).await?;
//let Rval{id} = Rval::from_row(&row).unwrap();
    Ok(id)
}

/// delete phone by its id
pub async fn delete_by_id<I>(
    pool: &sqlx::PgPool, 
    id: u32, 
   
) -> Result<Option<i32>, sqlx::Error>
where
    I: AsRef<str>,
{
    let Rval{id} = sqlx::query_as(&DELETE_PHONE_ID)
    .bind(id)
    .fetch_one(pool).await?;
    Ok(id)
}