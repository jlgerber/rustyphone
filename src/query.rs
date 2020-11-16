use crate::Person;
use crate::print_person;
use sqlx::FromRow;
use futures::TryStreamExt;
use serde::{Serialize, Deserialize};

pub async fn using_fetch_all(pool: &sqlx::PgPool, fields: &str, age_q: i16, sex: &str) -> Result<(),sqlx::Error> {
    let select = format!("SELECT {} FROM person where age>$1 and sex=$2",fields);
    let  names = sqlx::query_as::<_,Person>(&select)
    .bind(age_q)
    .bind(sex)
    .fetch_all(pool).await?;
    
    println!("{:#?}", names);
    Ok(())
}

pub async fn using_query(pool: &sqlx::PgPool, fields: &str, age_q: i16, sex: &str) -> Result<(), sqlx::Error> {
    let select = format!("SELECT {} FROM person where age>$1 and sex=$2",fields);
    let mut rows = sqlx::query(&select)
    .bind(age_q)
    .bind(sex)
    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let person = Person::from_row(&row).unwrap();   
        print_person(&person);
    
    }
    Ok(())
}

const QUERY: &'static str = r"WITH pview AS
( 
    SELECT * 
    FROM personview
    WHERE login='jdoe'
)
SELECT row_to_json(ln) AS personview 
FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login,
         ( SELECT json_agg(rowval) AS phones 
             FROM 
                ( SELECT phone_id, number, category, location 
                    FROM pview 
                   WHERE person_id = pv.person_id
                ) 
           rowval
          ) 
       FROM pview AS pv
     ) AS ln;";

#[derive(FromRow, Debug)]  
pub struct JV {
    pub personview: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Phone {
    phone_id: i32,
    number: String,
    category: String,
    location: String
} 

#[derive(Serialize, Deserialize, Debug )]
//#[sqlx(rename="personview")]
pub struct PersonView {
    person_id: i32,
    first: String,
    last: String,
    login: String,
    phones: Vec<Phone>//Vec<sqlx::types::Json<Phone>>
}

pub async fn json_back(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
  
    let mut rows = sqlx::query(&QUERY)
    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let person =JV::from_row(&row).unwrap();   
        let person: PersonView = serde_json::from_value(person.personview).unwrap();
        println!("{:#?}", person);
    
    }
    Ok(())
}