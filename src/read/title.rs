
use futures::TryStreamExt;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
const QUERY: &'static str = r"
SELECT 
    row_to_json(r) AS titleview 
FROM (
        SELECT 
            * 
        FROM
            title 
        ORDER BY 
            name
    ) AS r;";

// const QUERY_NAME: &'static str = r"
// WITH pview AS
// ( 
//     SELECT * 
//     FROM personview
//     WHERE fullname {comparison} $1
// )
// SELECT row_to_json(ln2) as personview from (
// SELECT DISTINCT ON (person_id) *  
// FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login, pv.fullname, pv.department, pv.title,
//         ( SELECT json_agg(rowval) AS phones 
//             FROM 
//                 ( SELECT 
//                         phone_id, number, category, location 
//                   FROM 
//                         pview 
//                   WHERE 
//                         person_id = pv.person_id
//                   AND
//                         pv.phone_id IS NOT NULL
//                 ) 
//             rowval
//         ) 
//         FROM pview AS pv
//     ) AS ln
// ) AS ln2;";
     
#[derive(Serialize,Deserialize,Debug)]
pub struct TitleView {
    pub id: i32,
    pub name: String,
}
// just a way of extracting the json. We need to be able to implement
// FromRow on something. (unless serde_json::Value has it implemented)
#[derive(FromRow, Debug)]  
struct JasonAdapter {
    pub titleview: serde_json::Value
}

/// Given a reference to the PgPool and a QueryParam instance, look up the 
/// matching values in the db and return a vector of json objects.
pub async fn titleview(pool: &sqlx::PgPool) -> Result<Vec<serde_json::Value>, sqlx::Error> {    
    let mut rval = Vec::new();
    let mut rows = sqlx::query(&QUERY)
                    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let JasonAdapter{titleview} =JasonAdapter::from_row(&row).unwrap();   
        rval.push(titleview);
    }
    Ok(rval)
}