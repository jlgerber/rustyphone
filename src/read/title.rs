
use futures::TryStreamExt;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::JsonAdapter;

const QUERY: &'static str = r"
SELECT 
    row_to_json(r) AS inner 
FROM (
        SELECT 
            * 
        FROM
            title 
        ORDER BY 
            name
    ) AS r;";

     
#[derive(Serialize,Deserialize,Debug)]
pub struct TitleView {
    pub id: i32,
    pub name: String,
}

/// Given a reference to the PgPool and a QueryParam instance, look up the 
/// matching values in the db and return a vector of json objects.
pub async fn query(pool: &sqlx::PgPool) -> Result<Vec<serde_json::Value>, sqlx::Error> {    
    let mut rval = Vec::new();
    let mut rows = sqlx::query(&QUERY)
                    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let JsonAdapter{inner} =JsonAdapter::from_row(&row).unwrap();   
        rval.push(inner);
    }
    Ok(rval)
}