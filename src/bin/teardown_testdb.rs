//use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use sqlx::postgres::PgConnectOptions;

static DROP: &str = r"
DROP DATABASE
    test
IF EXISTS;
CREATE DATABASE test;
";

pub async fn setup(
    mut pool: sqlx::PgConnection, 
) -> Result<(), sqlx::Error> {
    let   rows = sqlx::query(&DROP);
    
    // uncomment to print out query for debugging purposes
    // use sqlx::Execute;
    //println!("sql {}", rows.sql());
    let _ = rows.execute(&mut pool).await?;
                   
    
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let  pool = PgPoolOptions::new()
    //     .max_connections(1)
    //     .connect(DB_URL).await?;
    let conn = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .database("postgres")
        .username("postgres")
        .password("example")
        .connect().await?;
    
    let _ = setup(conn).await?; 
    Ok(())
}