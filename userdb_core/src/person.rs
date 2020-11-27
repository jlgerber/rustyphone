use sqlx::FromRow;

/// Person returned from a sql query
#[derive(FromRow, Debug)]
pub struct Person{
    pub id: i32, 
    #[sqlx(default)]
    pub first: Option<String>, 
    #[sqlx(default)]
    pub last: Option<String>, 
    #[sqlx(default)]
    pub login: Option<String>, 
}
