use sqlx::postgres::PgPoolOptions;
use async_std;

use userdb::{/*using_query, using_fetch_all, */query};
use userdb::DB_URL;
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

    let opt = Opt::from_args();
    if opt.name.is_none() && opt.login.is_none() {
        eprintln!("Error: Must provide --name or --login");
        std::process::exit(1);
    }
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
    // let fields = PersonSelect::all()
    // .login(false)
    // .select();
    // // query params
    // let age: i16 = 1;
    // let sex = "m";

    let Opt {
        name, login
    } = opt;

    
  let results = query(&pool,name, login).await?;
  for result in results {
      println!("{:#?}", result);
  }
  Ok(())
}
