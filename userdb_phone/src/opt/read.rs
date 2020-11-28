
use structopt::StructOpt;

use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::NumberString;

//-----------------------//
//          READ         //
//-----------------------//

#[derive(StructOpt, Debug)]
pub enum ReadOpt {
    /// Read contact information about people
    Person {
        /// Full name query
        #[structopt(short, long)]
        name: Option<String>,
    
        /// specify the name of your login
        #[structopt(short, long)]
        login: Option<String>,

        /// specify the title
        #[structopt(short, long)]
        title: Option<String>,

        /// specify the department
        #[structopt(short, long)]
        dept: Option<String>,

        /// Optionally specify the fullname (alternative to using --name flag)
        #[structopt(name="FULLNAME")]
        fullname: Option<String>,

        /// Display results as json instead of a table
        #[structopt(short,long)]
        json: bool,
    },
    Title {
        /// Display results as json instead of a table
        #[structopt(short,long)]
        json: bool,
    },
    Department {
        /// Display results as json instead of a table
        #[structopt(short,long)]
        json: bool,
    },
    Phone {
        /// specify the id
        #[structopt(short, long)]
        id: Option<u32>,

        /// Specify the number to match
        #[structopt(short, long)]
        number: Option<NumberString>,

        /// Specify the category of your phone number
        #[structopt(short, long)]
        category: Option<PhoneCategory>,

        /// Specify the location of your phone number
        #[structopt(short, long)]
        location: Option<Location>,

        /// Display results as json instead of as a table
        #[structopt(short,long)]
        json: bool,
    }
}