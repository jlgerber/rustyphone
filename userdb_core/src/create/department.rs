use std::convert::AsRef;
use sqlx::prelude::*;

const CREATE_DEPT: &str = r"
INSERT INTO department 
    (name)
VALUES
    ($1)
ON CONFLICT DO NOTHING
RETURNING id;
";

#[derive(FromRow)]
struct Rval {
    id: Option<i32>
}
pub async fn create<I>(pool: &sqlx::PgPool, name: I) 
-> Result<Option<i32>, sqlx::Error>
where
    I: AsRef<str>
{
    let Rval{id} = sqlx::query_as(&CREATE_DEPT)
    .bind(name.as_ref())
    .fetch_one(pool).await?;
    Ok(id)
}