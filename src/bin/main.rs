use sqlx::postgres::PgPoolOptions;
use async_std;

use userdb::read;
use userdb::create;
use userdb::DB_URL;
use userdb::QueryParam;
use userdb::PersonView;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(about="crud operations for phone command")]
enum Opt {
    /// create a person
    Create {

        ///provide first name
        #[structopt(name = "FIRSTNAME")]
        first: String,
        /// provide the last name
        #[structopt(name = "LASTNAME")]
        last: String,
        /// provide the login
        #[structopt(name = "LOGIN")]
        login: String,
        /// provide teh department
        #[structopt(name = "DEPARTMENT")]
        department: String,
        /// provide the title
        #[structopt(name = "TITLE")]
        title: String
    },
    /// query the userdb for a person or persons matching the 
    /// supplied argument
    Read {
        /// Full name query
        #[structopt(short, long)]
        name: Option<String>,
    
        /// specify the name of your login
        #[structopt(short, long)]
        login: Option<String>
    }
}

async fn process_read(name: Option<String>, login: Option<String>) -> Result<(),sqlx::Error> {
    // verify that either name or login is set
    if name.is_none() && login.is_none() {
        eprintln!("\nError: Must provide --name or --login");
        std::process::exit(1);
    }
    if name.is_some() && login.is_some() {
        eprintln!("\nError: must select either login or name");
        std::process::exit(1);
    }
    
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
        
    let results = read::personview(&pool,query_param).await?;
    println!("Result Cnt: {}", results.len());
    for result in results {
        let person: PersonView = serde_json::from_value(result).unwrap();
        //rval.push(person);
        println!("{:#?}", person);
    }
    Ok(())
}

async fn process_create(first: &str, last:&str, login: &str, department: &str, title: &str) 
-> Result<(),sqlx::Error> {
    let  pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await?;
        
    let result = create::person(&pool, first, last, login, department, title).await?;
    println!("ID: {}", result);
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // build options from structopt
    let opt = Opt::from_args();
    match opt {
        Opt::Read{name, login} => process_read(name, login).await,
        Opt::Create{first, last, login, department, title} 
            => process_create(&first, &last, &login, &department, &title).await
    }
}
