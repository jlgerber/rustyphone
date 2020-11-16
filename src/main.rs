use sqlx::postgres::PgPoolOptions;
use async_std;

use userdb::{PersonSelect};
use userdb::{using_query, using_fetch_all, json_back};
use userdb::DB_URL;
use userdb::Mode;
use userdb::prelude::*;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {

    let  pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await?;
    
    // you can either start off with a default PersonSelect and build up
    // additively like this
    // let fields = PersonSelect::new()
    //     .first(true)
    //     .last(true)
    //     .select();

    // or start off with a PersonSelect that is all on and
    // pare it down subtractively. 
    let fields = PersonSelect::all()
    .login(false)
    .select();
    // query params
    let age: i16 = 1;
    let sex = "m";

    let mode = Mode::Json;

    match mode {
        Mode::Query => using_query(&pool, &fields, age, sex).await,
        Mode::FetchAll => using_fetch_all(&pool, &fields, age, sex).await,
        Mode::Json => json_back(&pool).await
    }
}
