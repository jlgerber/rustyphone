
use sqlx::FromRow;
use futures::TryStreamExt;
use crate::PersonView;
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
    FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login, pv.fullname,
            ( SELECT json_agg(rowval) AS phones 
                FROM 
                    ( SELECT phone_id, number, category, location 
                        FROM pview 
                        WHERE person_id = pv.person_id
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
FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login, pv.fullname,
        ( SELECT json_agg(rowval) AS phones 
            FROM 
                ( SELECT phone_id, number, category, location 
                    FROM pview 
                    WHERE person_id = pv.person_id
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

/// Provides a tuple of query,mode 
#[derive(PartialEq, Eq, Debug)]
pub enum QueryParam {
    Name(String, QueryMode),
    Login(String, QueryMode)
}

impl QueryParam {
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
/// matching PersonView instances.
pub async fn query(pool: &sqlx::PgPool, param: QueryParam) -> Result<Vec<PersonView>, sqlx::Error> {
    let mut lookup = HashMap::new();
    let (query, binding) = match param {
        QueryParam::Name(name, mode) => {
            match mode {
                QueryMode::ILike => {
                    lookup.insert("comparison".into(), "ILIKE");
                    let query_name = strfmt(QUERY_NAME, &lookup).unwrap();
                    let name = format!("%{}%", name);
                    (query_name, name)
                },
                QueryMode::Like => {
                    lookup.insert("comparison".into(), "LIKE");
                    let query_name = strfmt(QUERY_NAME, &lookup).unwrap();
                    let name = format!("%{}%", name);
                    (query_name, name)
                },
                QueryMode::Exact => {
                    lookup.insert("comparison".into(), "=");
                    let query_name = strfmt(QUERY_NAME, &lookup).unwrap();
                    (query_name, name)
                },
            }
        }
        QueryParam::Login(login, mode) => {
            match mode {
                QueryMode::ILike => {
                    lookup.insert("comparison".into(), "ILIKE");
                    let query_login = strfmt(QUERY_LOGIN, &lookup).unwrap();
                    let login = format!("%{}%", login);
                    (query_login, login)
                },
                QueryMode::Like => {
                    lookup.insert("comparison".into(), "LIKE");
                    let query_login = strfmt(QUERY_LOGIN, &lookup).unwrap();
                    let login = format!("%{}%", login);
                    (query_login, login)
                },
                QueryMode::Exact => {
                    lookup.insert("comparison".into(), "=");
                    let query_login = strfmt(QUERY_LOGIN, &lookup).unwrap();
                    (query_login, login)
                },
            }
        }
    };
    
    let mut rval = Vec::new();
    let mut rows = sqlx::query(&query)
                    .bind(binding)
                    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let JasonAdapter{personview} =JasonAdapter::from_row(&row).unwrap();   
        let person: PersonView = serde_json::from_value(personview).unwrap();
        rval.push(person);
    
    }
    Ok(rval)
}