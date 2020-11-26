//! Delete a phone number given a person or person id. 
//! One would probably want to look at person_id::delete 
//! under normal circumstances...
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

/// Delete a phone record which matches the supplied arguments. 
/// This method is most likely NOT what you want. Please
/// use ```person_phone::delete``` instead.
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
    Ok(id)
}

/// delete phone given its id. This is a more direct method which 
/// simply deletes the phone by its id.
pub async fn delete_by_id(
    pool: &sqlx::PgPool, 
    id: u32, 
) -> Result<Option<i32>, sqlx::Error>
{
    let Rval{id} = sqlx::query_as(&DELETE_PHONE_ID)
    .bind(id)
    .fetch_one(pool).await?;
    Ok(id)
}