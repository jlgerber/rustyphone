
use sqlx::FromRow;
use futures::TryStreamExt;
use crate::PersonView;

const QUERY_LOGIN: &'static str = r"
WITH pview AS
( 
    SELECT * 
    FROM personview
    WHERE login LIKE $1
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
    WHERE fullname ILIKE $1
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


pub async fn query(pool: &sqlx::PgPool, name: Option<String>, login: Option<String>) -> Result<Vec<PersonView>, sqlx::Error> {
    let (query,binding) = if let Some(name) = name {
        ( QUERY_NAME, name)
    } else if let Some(login) = login {
        (QUERY_LOGIN, login) 
    } else {
        panic!("Should not reach here")
    };
    let mut rval = Vec::new();
    let binding = format!("%{}%", binding);
    let mut rows = sqlx::query(query)
    .bind(binding)
    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let JasonAdapter{personview} =JasonAdapter::from_row(&row).unwrap();   
        let person: PersonView = serde_json::from_value(personview).unwrap();
        rval.push(person);
    
    }
    Ok(rval)
}