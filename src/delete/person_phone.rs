//! Delete a person's phone number. More accurately, delete the association between
//! a phone number and a person, since numbers may be shared (like extensions)
use std::convert::AsRef;
use sqlx::prelude::*;
use crate::prelude::*;

const DELETE_PHONE: &str = r"
WITH phone_cte AS (
    SELECT 
        phone_id,
        person_id
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
    people_phones 
WHERE
    people_phones.person_id in (
        SELECT 
            person_id 
        FROM 
            phone_cte
    )
AND
    people_phones.phone_id in (
        SELECT
            phone_id
        FROM
            phone_cte
    )
returning phone_id
";

#[derive(FromRow)]
struct Rval {
    phone_id: Option<i32>
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
    let Rval{phone_id} = sqlx::query_as(&DELETE_PHONE)
    .bind(login.as_ref())
    .bind(number)
    .bind(category.to_static_str())
    .bind(location.to_static_str())
    .fetch_one(pool).await?;
//let Rval{id} = Rval::from_row(&row).unwrap();
    Ok(phone_id)
}
