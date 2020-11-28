
use futures::TryStreamExt;
use sqlx::FromRow;
use std::collections::HashMap;
use strfmt::strfmt;
use crate::QueryMode;
use crate::JsonAdapter;

/// Struct encapsulating potential query parameters
#[derive(Debug)]
pub struct SimplePersonQuery {
   pub value: String,
}

impl SimplePersonQuery {

    /// Generate a prepared statement to query for person(s) as a string
    pub fn query(&self, mode: &QueryMode) -> String {
        let mut lookup = HashMap::new();
       
        let where_clause = format!("WHERE fullname {} $1 OR login {} $1 or title {} $1 OR department {} $1", mode, mode, mode, mode);
         
        lookup.insert("query".into(), where_clause);
        strfmt(QUERY, &lookup).unwrap()
    }

    pub fn new<I>(value: I) -> Self 
    where 
        I: Into<String>
    {
        SimplePersonQuery{
            value: value.into()
        }
    }

}

const QUERY: &'static str = r"
WITH pview AS
( 
    SELECT * 
    FROM personview
    {query}
)
SELECT row_to_json(ln2) as inner from (
    SELECT DISTINCT ON (person_id) *  
    FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login, pv.fullname, pv.department, pv.title,
            ( SELECT 
                json_agg(rowval) AS phones 
              FROM 
                    ( SELECT phone_id, number, category, location 
                        FROM 
                            pview 
                        WHERE 
                            person_id = pv.person_id
                        AND
                            pv.phone_id IS NOT NULL
                    ) 
                rowval
            ) 
            FROM pview AS pv
        ) AS ln
) AS ln2;";


/// Given a SimplePersonQuery instance and a mode, retrieve the results from the database
pub async fn query(
    pool: &sqlx::PgPool, 
    query: SimplePersonQuery, 
    mode: QueryMode
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let mut rval = Vec::new();
    let querymode = query.query(&mode);
    let  mut rows = sqlx::query(&querymode);
    let SimplePersonQuery{mut value} = query;

    if mode == QueryMode::ILike || mode == QueryMode::Like {
        value = format!("%{}%", value);
    }

    rows = rows.bind(value);
    
    let mut rows = rows.fetch(pool);
                   
    while let Some(row) = rows.try_next().await? {
        let JsonAdapter{inner} =JsonAdapter::from_row(&row).unwrap();   
        rval.push(inner);
    }
    Ok(rval)
}