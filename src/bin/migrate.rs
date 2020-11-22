//use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
//use sqlx::migrate;
use sqlx::postgres::PgConnectOptions;
//const DB_URL: &'static str = "postgres://postgres:example@localhost:5432/test";
use async_std;

use sqlx::migrate::Migrator;
static MIGRATOR: Migrator = sqlx::migrate!();

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let  pool = PgPoolOptions::new()
    //     .max_connections(1)
    //     .connect(DB_URL).await?;
    let mut conn = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres").password("example").connect().await?;

        for m in MIGRATOR.iter() {
            println!("version {}", &m.version);
            println!("description {}", &m.description);
            println!("{:?}", &m.sql);
        }
    MIGRATOR.run(&mut conn).await?;
    // migrate!("./mig")
    // .run(&mut conn)
    // .await?;

    Ok(())
}