//! phone command
//!
//! Simple cli for finding users in the userdb.

use structopt::StructOpt;

// internal
use userdb_phone::process::read::process_read_simple_person;


#[derive(StructOpt, Debug)]
#[structopt(about="Search for people...")]
pub struct Opt {
    /// Specifcy the name, login, department, or title to search on.
    #[structopt(name="VALUE")]
    pub value: String,
    /// Display results as json instead of as a table
    #[structopt(short,long)]
    pub json: bool,
}


#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // build options from structopt
    let opt = Opt::from_args();
    match opt {
        Opt{value, json} => {
            process_read_simple_person(value, json ).await
        },
    }
    
}
