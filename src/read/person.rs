
use futures::TryStreamExt;
use sqlx::FromRow;
use std::collections::HashMap;
use strfmt::strfmt;
use std::fmt;

/// Struct encapsulating potential query parameters
#[derive(Debug)]
pub struct PersonQuery {
    pub name: Option<String>,
    pub login: Option<String>,
    pub title: Option<String>,
    pub dept: Option<String>
}

impl Default for PersonQuery {
    fn default() -> Self {
        Self {
            name: None,
            login: None,
            title: None,
            dept: None
        }
    }
}
impl PersonQuery {

    /// Generate a prepared statement to query for person(s) as a string
    pub fn query(&self, mode: &QueryMode) -> String {
        let mut lookup = HashMap::new();
        let mut where_clause = String::new();
        // start with 1 as the $var in postgres's prepared statements 
        // start at $1
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
            cnt +=1;
        }
        if self.dept.is_some() {
            where_clause = format!("{}\n{} department {} ${}", where_clause, Self::joiner(cnt), mode, cnt);
            //cnt +=1;
        }
        lookup.insert("query".into(), where_clause);
        strfmt(QUERY, &lookup).unwrap()
    }

    pub fn new() -> Self {
        PersonQuery::default()
    }

    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn login(mut self, login: Option<String>) -> Self {
        self.login = login;
        self
    }

    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }
    
    pub fn dept(mut self, dept: Option<String>) -> Self {
        self.dept = dept;
        self
    }
    // little helper function to build up the query
    fn joiner(cnt: u8) -> &'static str {
        // 1 is the lowest value that we should encounter, since
        // the index is 1-based.
        if cnt == 1 {
            "WHERE"
        } else {
            "AND"
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
/// Given a PersonQuery instance and a mode, retrieve the results from the database
pub async fn query(
    pool: &sqlx::PgPool, 
    query: PersonQuery, 
    mode: QueryMode
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let mut rval = Vec::new();
    let querymode = query.query(&mode);
    let  mut rows = sqlx::query(&querymode);
    let PersonQuery{name, login, title, dept} = query;
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
    if dept.is_some() {
        let mut dept = dept.unwrap();
        if mode == QueryMode::ILike || mode == QueryMode::Like {
            dept = format!("%{}%", dept);
        }
        rows = rows.bind(dept);
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