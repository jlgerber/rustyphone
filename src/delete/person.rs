//! Delete a person given a login or  id. 
use std::convert::AsRef;
use sqlx::prelude::*;

const DELETE: &str = r"
SELECT 
    * 
FROM
    deletePerson($1) as id;
";

const DELETE_BY_ID: &str = r"
SELECT 
    *
FROM
    deletePersonById($1) as id;
";

#[derive(FromRow)]
struct Rval {
    id: Option<i32>
}

/// Delete a person record which matches the supplied arguments. 
pub async fn delete<I>(
    pool: &sqlx::PgPool, 
    login: I, 
) -> Result<Option<i32>, sqlx::Error>
where
    I: AsRef<str>,
{
    let Rval{id} = sqlx::query_as(&DELETE)
    .bind(login.as_ref())
    .fetch_one(pool).await?;
    if let Some(value) = id {
        if value == 0 {
            return Ok(None);
        }
    } 
    Ok(id)
}

/// delete phone given its id. This is a more direct method which 
/// simply deletes the phone by its id.
pub async fn delete_by_id(
    pool: &sqlx::PgPool, 
    id: u32, 
) -> Result<Option<i32>, sqlx::Error> {

    let Rval{id} = sqlx::query_as(&DELETE_BY_ID)
    .bind(id)
    .fetch_one(pool).await?;
    if let Some(value) = id {
        if value == 0 {
            return Ok(None);
        }
    } 
    Ok(id)
}