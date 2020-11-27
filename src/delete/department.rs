//! Delete a person's phone number. More accurately, delete the association between
//! a phone number and a person, since numbers may be shared (like extensions)
use std::convert::AsRef;
use sqlx::prelude::*;
//use crate::prelude::*;

const DELETE_DEPARTMENT: &str = r"
SELECT 
    * 
FROM 
    deleteDepartment($1) AS dept_id;
";

const DELETE_DEPT_FROM_IDS: &str = r"
SELECT
    *
FROM
    deleteDepartmentById($1::INT) AS dept_id;
";

#[derive(FromRow)]
struct Rval {
    dept_id: Option<i32>
}

/// Deletes the association between phone number and person. 
/// If there are no other persons associated with the phone number
/// delete the underlying phone number record as well.
pub async fn delete<I>(
    pool: &sqlx::PgPool, 
    department: I, 
) -> Result<Option<i32>, sqlx::Error>
where
    I: AsRef<str>,
{
    let Rval{dept_id} = sqlx::query_as(&DELETE_DEPARTMENT)
    .bind(department.as_ref())
    .fetch_one(pool).await?;
    if let Some(value) = dept_id {
        if value == 0 {
            return Ok(None);
        }
    } 
    Ok(dept_id)
}

/// Deletes the association between phone number and person. 
/// If there are no other persons associated with the phone number
/// delete the underlying phone number record as well.
pub async fn delete_by_id(
    pool: &sqlx::PgPool, 
    dept_id: u32,
) -> Result<Option<i32>, sqlx::Error>
{
    let Rval{dept_id} = sqlx::query_as(&DELETE_DEPT_FROM_IDS)
    .bind(dept_id)
    .fetch_one(pool).await?;
    if let Some(value) = dept_id {
        if value == 0 {
            return Ok(None);
        }
    } 
    Ok(dept_id)
}

