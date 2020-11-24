//! C in crud
//! create

//use crate::PersonView;
use std::convert::AsRef;
use sqlx::prelude::*;

const CREATE_PERSON: &'static str = r"
WITH cte_department AS (
    SELECT 
        id as dept_id
    FROM
        department
    WHERE
        name = $1
),
cte_title AS (
    SELECT
        id as title_id
    FROM
        title
    WHERE
        name = $2
)
INSERT INTO person 
    (first, last, login, department_id, title_id)
VALUES
    ($3, $4, $5, 
        (SELECT dept_id FROM cte_department), 
        (SELECT title_id FROM cte_title))
RETURNING id;
";

#[derive(FromRow)]
struct Rval {
    id: i32
}
pub async fn person<I, J, K, L, M>(pool: &sqlx::PgPool, first: I, last: J, login: K, department: L, title: M) 
-> Result<i32, sqlx::Error>
where
    I: AsRef<str>,
    J: AsRef<str>,
    K: AsRef<str>,
    L: AsRef<str>,
    M: AsRef<str>,
{
    let row = sqlx::query(&CREATE_PERSON)
    .bind(department.as_ref())
    .bind(title.as_ref())
    .bind(first.as_ref())
    .bind(last.as_ref())
    .bind(login.as_ref())
    .fetch_one(pool).await?;
    let Rval{id} = Rval::from_row(&row).unwrap();
    Ok(id)
}