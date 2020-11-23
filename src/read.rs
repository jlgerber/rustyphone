
use sqlx::FromRow;
use futures::TryStreamExt;
//use crate::PersonView;
use strfmt::strfmt;
use std::collections::HashMap;

const QUERY_LOGIN: &'static str = r"
WITH pview AS
( 
    SELECT * 
    FROM personview
    WHERE login {comparison} $1
)
SELECT row_to_json(ln2) as personview from (
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

const QUERY_NAME: &'static str = r"
WITH pview AS
( 
    SELECT * 
    FROM personview
    WHERE fullname {comparison} $1
)
SELECT row_to_json(ln2) as personview from (
SELECT DISTINCT ON (person_id) *  
FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login, pv.fullname, pv.department, pv.title,
        ( SELECT json_agg(rowval) AS phones 
            FROM 
                ( SELECT 
                        phone_id, number, category, location 
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
     
// just a way of extracting the json. We need to be able to implement
// FromRow on something. (unless serde_json::Value has it implemented)
#[derive(FromRow, Debug)]  
struct JasonAdapter {
    pub personview: serde_json::Value
}


/// The query mode identified how the receiver should
/// treat the requested query. 
/// - ILike tests to see if the supplied param is a substring of 
///   the target value, ignoring case.
/// - Like works like `ILike` but pays attention to case
/// - Exact matches exactly
#[derive(Debug, PartialEq, Eq)]
pub enum QueryMode {
    ILike,
    Like,
    Exact
}
impl QueryMode {
    /// Return the comparsion operator as a static str 
    pub fn comparison(&self) -> &'static str {
        match self {
            &Self::ILike => "ILIKE",
            &Self::Like => "LIKE",
            &Self::Exact => "=",
        }
    }
}
/// The type of field which we wish to query
#[derive(PartialEq, Eq, Debug)]
pub enum QueryField {
    Name, 
    Login
}
/// Provides a tuple of query, mode 
#[derive(PartialEq, Eq, Debug)]
pub enum QueryParam {
    Name(String, QueryMode),
    Login(String, QueryMode)
}

impl QueryParam {
    /// Given a value, field, and mode, create a new QueryParam
    pub fn new<I:Into<String>>(value: I, field: QueryField, mode: QueryMode) -> Self {
        match field {
            QueryField::Name  => Self::Name( value.into(), mode),
            QueryField::Login => Self::Login(value.into(), mode)
        }
    }

    /// Given a name, construct a param which expects to match any string
    /// which contains the name, irrespective of case.
    pub fn ilike_name<I:Into<String>>(name: I) -> Self {
        Self::Name(name.into(), QueryMode::ILike)
    }

    /// Given a login, constuct a param which expects to match any string
    /// which contains the supplied login, irrespective of case.
    pub fn ilike_login<I>(login: I) -> Self where I: Into<String> {
        Self::Login(login.into(), QueryMode::ILike)
    }

    /// Given a name, construct a param which expects to match any string
    /// which contains the name
    pub fn like_name<I:Into<String>>(name: I) -> Self {
        Self::Name(name.into(), QueryMode::Like)
    }

    /// Given a login, constuct a param which expects to match any string
    /// which contains the supplied login.
    pub fn like_login<I>(login: I) -> Self where I: Into<String> {
        Self::Login(login.into(), QueryMode::Like)
    }

    /// given a name, construct a param which expects to match it exactly
    pub fn exact_name<I:Into<String>>(name: I) -> Self {
        Self::Name(name.into(), QueryMode::Exact)
    }

    /// Given a login, construct a param which expects to match the supplied
    /// login exactly.
    pub fn exact_login<I>(login: I) -> Self where I: Into<String> {
        Self::Login(login.into(), QueryMode::Exact)
    }
}

/// Given a reference to the PgPool and a QueryParam instance, look up the 
/// matching values in the db and return a vector of json objects.
pub async fn personview(pool: &sqlx::PgPool, param: QueryParam) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let mut lookup = HashMap::new();
    let (query, binding) = match param {
        QueryParam::Name(name, mode) => {
            lookup.insert("comparison".into(), mode.comparison());
            let query_name = strfmt(QUERY_NAME, &lookup).unwrap();
            let name = format!("%{}%", name);
            (query_name, name)
        }
        QueryParam::Login(login, mode) => {
            lookup.insert("comparison".into(), mode.comparison());
            let query_login = strfmt(QUERY_LOGIN, &lookup).unwrap();
            let login = format!("%{}%", login);
            (query_login, login)
        }
    };
    
    let mut rval = Vec::new();
    let mut rows = sqlx::query(&query)
                    .bind(binding)
                    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let JasonAdapter{personview} =JasonAdapter::from_row(&row).unwrap();   
        //let person: PersonView = serde_json::from_value(personview).unwrap();
        //rval.push(person);
        rval.push(personview);
    
    }
    Ok(rval)
}