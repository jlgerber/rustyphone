//! Delete a person's phone number. More accurately, delete the association between
//! a phone number and a person, since numbers may be shared (like extensions)
use std::convert::AsRef;
use sqlx::prelude::*;
use crate::prelude::*;

const DELETE_PHONE: &str = r"
SELECT 
    * 
FROM 
    deletePhone($1, $2, $3::phonecategory, $4::location) AS phone_id;
";

const DELETE_PHONE_FROM_IDS: &str = r"
SELECT
    *
FROM
    deletePhoneFromIds($1, $2) AS phone_id;
";
#[derive(FromRow)]
struct Rval {
    phone_id: Option<i32>
}

/// Deletes the association between phone number and person. 
/// If there are no other persons associated with the phone number
/// delete the underlying phone number record as well.
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
    let Rval{phone_id} = sqlx::query_as(&DELETE_PHONE)
    .bind(login.as_ref())
    .bind(number)
    .bind(category.to_static_str())
    .bind(location.to_static_str())
    .fetch_one(pool).await?;
//let Rval{id} = Rval::from_row(&row).unwrap();
    Ok(phone_id)
}

/// Deletes the association between phone number and person. 
/// If there are no other persons associated with the phone number
/// delete the underlying phone number record as well.
pub async fn delete_from_ids(
    pool: &sqlx::PgPool, 
    person_id: u32,
    phone_id: u32,
) -> Result<Option<i32>, sqlx::Error>
{
    let Rval{phone_id} = sqlx::query_as(&DELETE_PHONE_FROM_IDS)
    .bind(person_id)
    .bind(phone_id)
    .fetch_one(pool).await?;
//let Rval{id} = Rval::from_row(&row).unwrap();
    Ok(phone_id)
}

