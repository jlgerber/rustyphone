//! Delete a person's phone number. More accurately, delete the association between
//! a phone number and a person, since numbers may be shared (like extensions)
use std::convert::AsRef;
use sqlx::prelude::*;
//use crate::prelude::*;

const DELETE_TITLE: &str = r"
SELECT 
    * 
FROM 
    deleteTitle($1) AS dept_id;
";

const DELETE_TITLE_FROM_IDS: &str = r"
SELECT
    *
FROM
    deleteTitleById($1::INT) AS dept_id;
";

#[derive(FromRow)]
struct Rval {
    dept_id: Option<i32>
}

/// Deletes title if no persons hold it. 
pub async fn delete<I>(
    pool: &sqlx::PgPool, 
    title: I, 
) -> Result<Option<i32>, sqlx::Error>
where
    I: AsRef<str>,
{
    let Rval{dept_id} = sqlx::query_as(&DELETE_TITLE)
    .bind(title.as_ref())
    .fetch_one(pool).await?;
    Ok(dept_id)
}

/// Deletes the title by id, as long as no person holds the title.
pub async fn delete_by_id(
    pool: &sqlx::PgPool, 
    dept_id: u32,
) -> Result<Option<i32>, sqlx::Error>
{
    let Rval{dept_id} = sqlx::query_as(&DELETE_TITLE_FROM_IDS)
    .bind(dept_id)
    .fetch_one(pool).await?;
    if let Some(value) = dept_id {
        if value == 0 {
            return Ok(None);
        }
    } 
    Ok(dept_id)
}

