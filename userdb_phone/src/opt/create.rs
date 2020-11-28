
use structopt::StructOpt;

use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::NumberString;

//-------------------//
//       CREATE      //
//-------------------//

#[derive(StructOpt, Debug)]
pub enum CreateOpt {
    Person {

        /// Provide first name
        #[structopt(name = "FIRSTNAME")]
        first: String,

        /// Provide the last name
        #[structopt(name = "LASTNAME")]
        last: String,

        /// Provide the login
        #[structopt(name = "LOGIN")]
        login: String,

        /// Provide the department
        #[structopt(name = "DEPARTMENT")]
        department: String,

        /// Provide the title
        #[structopt(name = "TITLE")]
        title: String
    },
    Phone {

        /// specify the name of your login
        #[structopt(short="u", long)]
        login: String,

        /// Specify the number to match
        #[structopt(short, long)]
        number: NumberString,

        /// Specify the category of your phone number
        #[structopt(short, long)]
        category: PhoneCategory,

        /// Specify the location of your phone number
        #[structopt(short, long)]
        location: Location,

    },
    Title {
        /// Specify the name
        #[structopt(name = "TITLE")]
        title: String,
    },
    Department {
        /// Specify the name
        #[structopt(name = "DEPARTMENT")]
        department: String,
    }
}