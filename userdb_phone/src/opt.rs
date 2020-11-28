// submodules
mod create;
mod read;
mod update;
mod delete;
// import the contents of the submodules
// so that they can be imported from `opt`
pub use create::*;
pub use read::*;
pub use update::*;
pub use delete::*;

use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(about="Search for people, and more...")]
pub struct Opt {

        /// Fetch phone records for people by full name
        #[structopt(short, long)]
        pub name: Option<String>,
    
        /// Fetch phone records for people by login
        #[structopt(short, long)]
        pub login: Option<String>,

        /// Fetch phone records for people by title
        #[structopt(short, long)]
        pub title: Option<String>,

        /// Fetch phone records for people by department
        #[structopt(short, long)]
        pub dept: Option<String>,

        /// Optionally specify the fullname (alternative to using --name flag)
        #[structopt(name="FULLNAME")]
        pub fullname: Option<String>,

        /// Display results as json instead of as a table
        #[structopt(short,long)]
        pub json: bool,

        /// Optional subcommands
        #[structopt(subcommand)]
        pub cmd: Option<OptSub>
}


//--------------------------//
//      Subcommands         //
//--------------------------//  
#[derive(StructOpt, Debug)]
pub enum OptSub {

    /// Create entities
    Create {
        #[structopt(subcommand)]
        sub: CreateOpt,
    },

    /// Query entities (like title and department)
    Read {
        #[structopt(subcommand)]
        sub: ReadOpt,
    },
    /// Update existing entities
    Update {
        #[structopt(subcommand)]
        sub: UpdateOpt,
    },
    /// Delete existing entities
    Delete {
        #[structopt(subcommand)]
        sub: DeleteOpt,
    }
}

