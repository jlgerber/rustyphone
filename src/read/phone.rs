//! Read phone numbers
use std::collections::HashMap;
use strfmt::strfmt;
use futures::TryStreamExt;
use sqlx::FromRow;
use crate::PhoneCategory;
//use crate::PhoneNumber;
use crate::NumberString;
use crate::Location;
use crate::QueryMode;
use crate::prelude::*;
use crate::JsonAdapter;
use crate::where_joiner;

const QUERY: &'static str = r"
SELECT 
    row_to_json(r) AS inner 
FROM (
        SELECT 
            id as phone_id, * 
        FROM
            phone 
        {query}
    ) AS r;";


/// The parameters for a phone query.
#[derive(Debug,PartialEq, Eq)]
pub struct PhoneQuery {
    pub id: Option<u32>,
    pub number: Option<NumberString>,
    pub category: Option<PhoneCategory>,
    pub location: Option<Location>
}

impl Default for PhoneQuery {
    fn default() -> Self {
        Self {
            id: None,
            number: None,
            category: None,
            location: None
        }
    }
}

impl Queryable for PhoneQuery {
    fn query(&self, mode: &QueryMode) -> String {
        let mut lookup = HashMap::new();
        let mut where_clause = String::new();
        // start with 1 as the $var in postgres's prepared statements 
        // start at $1
        let mut cnt = 1;
        if self.id.is_some() {
            where_clause = format!("{} id = ${}::integer", where_joiner(cnt), cnt);
            cnt +=1;
        }
        if self.number.is_some() {
            where_clause = format!("{}\n{} number {} ${}", where_clause, where_joiner(cnt), mode, cnt);
            cnt+=1;
        }
        if self.category.is_some() {
            // now that we are using PhoneCategory instead of a string, we dont need LOWER()
            //where_clause = format!("{}\n{} category = LOWER(${})::phonecategory", where_clause, where_joiner(cnt), cnt);
            where_clause = format!("{}\n{} category = ${}::phonecategory", where_clause, where_joiner(cnt), cnt);
            cnt +=1;
        }
        if self.location.is_some() {
            // see category comment
            //where_clause = format!("{}\n{} location = LOWER(${})::location", where_clause, where_joiner(cnt),  cnt);
            where_clause = format!("{}\n{} location = ${}::location", where_clause, where_joiner(cnt),  cnt);
            //cnt +=1;
        }
        lookup.insert("query".into(), where_clause);
        strfmt(QUERY, &lookup).unwrap()
    }
}
impl PhoneQuery {
    /// New up an empty PhoneQuery intance
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the id on self, returning self as per the 
    /// owning builder pattern.
    pub fn id(mut self, id: Option<u32>) -> Self {
        self.id = id;
        self
    }
    /// Set the number on self and return self, as per the
    /// owning builder pattern.
    pub fn number(mut self, number: Option<NumberString>) -> Self {
        self.number = number;
        self
    }
    /// Set the category on self and return self, as per the 
    /// owning builder pattern.
    pub fn category(mut self, category: Option<PhoneCategory>) -> Self {
        self.category = category;
        self
    }
    /// Set the location on self and return self, as per the
    /// owning builder pattern.
    pub fn location(mut self, location: Option<Location>) -> Self {
        self.location = location;
        self
    }
}

/// Given a PersonQuery instance and a mode, retrieve the results from the database
pub async fn query(
    pool: &sqlx::PgPool, 
    query: PhoneQuery, 
    mode: QueryMode
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let mut rval = Vec::new();
    let querymode = query.query(&mode);
    let  mut rows = sqlx::query(&querymode);
    let PhoneQuery{id, number, category, location} = query;
    let id = id.map(|v| v.to_string());
    let category = category.map(|c| c.to_static_str().to_string());
    let location = location.map(|l| l.to_static_str().to_string());

    if id.is_some() {
        let id = id.unwrap();
        rows = rows.bind(id);
    }
    if number.is_some() {
        //let mut number = number.unwrap().inner();
        let mut number = number.unwrap().to_string();
        if mode == QueryMode::ILike || mode == QueryMode::Like {
            number = format!("%{}%", number);
        }
        rows = rows.bind(number);
    }
    if category.is_some() {
        let  category = category.unwrap();
        rows = rows.bind(category);
    }
    if location.is_some() {
        let  location = location.unwrap();
        rows = rows.bind(location);
    }
    // uncomment to print out query for debugging purposes
    // use sqlx::Execute;
    //println!("sql {}", rows.sql());
    let mut rows = rows.fetch(pool);
                   
    while let Some(row) = rows.try_next().await? {
        let JsonAdapter{inner} = JsonAdapter::from_row(&row).unwrap();   
        rval.push(inner);
    }
    Ok(rval)
}