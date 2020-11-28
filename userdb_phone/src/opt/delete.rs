
use structopt::StructOpt;
// internal
use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::NumberString;

//------------------//
//      DELETE      //
//------------------//

#[derive(StructOpt, Debug)]
pub enum DeleteOpt {
    Phone {
        /// delete by phone id
        #[structopt(short,long)]
        id: Option<u32>,

        /// specify the name of your login
        #[structopt(
            short="u", long,
            requires_all = &["number", "category", "location"]
        )]
        login: Option<String>,

        /// Specify the number to match
        #[structopt(
            short, 
            long,
            requires_all = &["login", "category", "location"]
        )]
        number: Option<NumberString>,

        /// Specify the category of your phone number
        #[structopt(
            short, 
            long,
            requires_all = &["login", "number", "location"]
        )]
        category: Option<PhoneCategory>,

        /// Specify the location of your phone number
        #[structopt(
            short, 
            long,
            requires_all = &["login", "number", "category"]
        )]
        location: Option<Location>,
    },
    Person {
         /// Name of the department to delete
         #[structopt(short, long)]
         login: Option<String>,
 
         /// Id of the department to delete
         #[structopt(short, long)]
         id: Option<u32>
    },

    Department {
        /// Name of the department to delete
        #[structopt(short, long)]
        name: Option<String>,

        /// Id of the department to delete
        #[structopt(short, long)]
        id: Option<u32>
    },
    Title {
        /// Name of the title to delete
        #[structopt(short, long)]
        name: Option<String>,

        /// Id of the title to delete
        #[structopt(short, long)]
        id: Option<u32>
    }
}