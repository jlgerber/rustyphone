//! C in crud
//! create

//use crate::PersonView;
use std::convert::AsRef;
use sqlx::prelude::*;

const CREATE_PHONE: &str = r"
SELECT 
    *
FROM
    addPhone($1, $2, $3, $4)
";

#[derive(FromRow)]
struct Rval {
    id: i32
}
pub async fn phone<I, J, K, L, M>(
    pool: &sqlx::PgPool, 
    login: I, 
    number: J, 
    category: K, 
    location: L
) -> Result<i32, sqlx::Error>
where
    I: AsRef<str>,
    J: AsRef<str>,
    K: AsRef<str>,
    L: AsRef<str>,
{
    let row = sqlx::query(&CREATE_PERSON)
    .bind(login.as_ref())
    .bind(number.as_ref())
    .bind(category.as_ref())
    .bind(location.as_ref())
    .fetch_one(pool).await?;
    let Rval{id} = Rval::from_row(&row).unwrap();
    Ok(id)
}