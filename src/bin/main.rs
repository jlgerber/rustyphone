use sqlx::postgres::PgPoolOptions;
use async_std;

use userdb::query;
use userdb::DB_URL;
use userdb::QueryParam;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "query")]
struct Opt {
    /// Full name query
    #[structopt(short, long)]
    name: Option<String>,

    /// specify the name of your login
    #[structopt(short, long)]
    login: Option<String>
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // build options from structopt
    let opt = Opt::from_args();
    // verify that either name or login is set
    if opt.name.is_none() && opt.login.is_none() {
        eprintln!("\nError: Must provide --name or --login");
        std::process::exit(1);
    }
    if opt.name.is_some() && opt.login.is_some() {
        eprintln!("\nError: must select either login or name");
        std::process::exit(1);
    }
    // destructure into constituent parts
    let Opt {
        name, login
    } = opt;
    // build out the query param, assuming that if name is set,
    // then login is not set
    let query_param = if name.is_some() {
        QueryParam::ilike_name(name.unwrap())
    } else {
        QueryParam::ilike_login(login.unwrap())
    };
    // construct a connection pool to the db
    let  pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await?;
        
    let results = query(&pool,query_param).await?;
    for result in results {
        println!("{:#?}", result);
    }
    Ok(())
}
