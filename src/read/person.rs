
use futures::TryStreamExt;
use sqlx::FromRow;
use std::collections::HashMap;
use strfmt::strfmt;
use std::fmt;

#[derive(Debug)]
pub struct PersonQuery {
    pub name: Option<String>,
    pub login: Option<String>,
    pub title: Option<String>
}

impl PersonQuery {

    fn joiner(cnt: u8) -> &'static str {
        if cnt == 1 {
            "WHERE"
        } else {
            "AND"
        }
    }
    pub fn query(&self, mode: &QueryMode) -> String {
        let mut lookup = HashMap::new();
        let mut where_clause = String::new();
        let mut cnt = 1;
        if self.name.is_some() {
            where_clause = format!("{} fullname {} ${}", Self::joiner(cnt), mode, cnt);
            
            cnt +=1;
        }
        if self.login.is_some() {
            where_clause = format!("{}\n{} login {} ${}", where_clause, Self::joiner(cnt), mode, cnt);
            cnt+=1;
        }
        if self.title.is_some() {
            where_clause = format!("{}\n{} title {} ${}", where_clause, Self::joiner(cnt), mode, cnt);
            //cnt +=1;
        }
        lookup.insert("query".into(), where_clause);
        strfmt(QUERY, &lookup).unwrap()
    }
}

const QUERY: &'static str = r"
WITH pview AS
( 
    SELECT * 
    FROM personview
    {query}
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
impl fmt::Display for QueryMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::ILike => write!(f, "ILIKE"),
            &Self::Like => write!(f, "LIKE"),
            &Self::Exact => write!(f, "="),
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

pub async fn view(
    pool: &sqlx::PgPool, 
    query: PersonQuery, 
    mode: QueryMode
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let mut rval = Vec::new();
    let querymode = query.query(&mode);
    let  mut rows = sqlx::query(&querymode);
    let PersonQuery{name, login, title} = query;
    if name.is_some() {
        let mut name = name.unwrap();
        if mode == QueryMode::ILike || mode == QueryMode::Like {
            name = format!("%{}%", name);
        }
        rows = rows.bind(name);
    }
    if login.is_some() {
        let mut login = login.unwrap();
        if mode == QueryMode::ILike || mode == QueryMode::Like {
            login = format!("%{}%", login);
        }
        rows = rows.bind(login);
    }
    if title.is_some() {
        let mut title = title.unwrap();
        if mode == QueryMode::ILike || mode == QueryMode::Like {
            title = format!("%{}%", title);
        }
        rows = rows.bind(title);
    }
    let mut rows = rows.fetch(pool);
                    //.bind(binding)
                    //.fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let JasonAdapter{personview} =JasonAdapter::from_row(&row).unwrap();   
        rval.push(personview);
    }
    Ok(rval)
}