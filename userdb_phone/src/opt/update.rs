
use structopt::StructOpt;
// internal
use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::NumberString;

//-------------------------//
//          UPDATE         //
//-------------------------//

#[derive(StructOpt, Debug)]
pub enum UpdateOpt {
    Person {
        /// Provide the person's id to apply updates to. 
        /// One may also select by login using --from-login.
        #[structopt(short, long)]
        id: Option<i32>,

        /// Provide the target login to apply the updates to, 
        /// as an alternative to --id
        #[structopt(short, long="from-login")]
        from_login: Option<String>,

        /// Optionally provide first name update
        #[structopt(short, long )]
        first: Option<String>,

        /// Optionally provide a last name update
        #[structopt(short, long )]
        last: Option<String>,

        /// Optionally provide a login update
        #[structopt(short="u", long )]
        login: Option<String>,

        /// Optionally provide a department id update
        #[structopt(short, long = "dept-id" )]
        department: Option<i32>,

        /// Optionally provide a title id update
        #[structopt(short, long="title-id" )]
        title: Option<i32>,
    },
    Phone {
        /// Provide the phone's id to apply updates to. 
        #[structopt(short, long)]
        id: i32,

        /// Optionally provide number update
        #[structopt(short, long )]
        number: Option<NumberString>,

        /// Optionally provide a category update
        #[structopt(short, long )]
        category: Option<PhoneCategory>,

        /// Optionally provide a location update
        #[structopt(short="u", long )]
        location: Option<Location>,
    },
    Title {
        /// Provide the title's id to apply updates to. 
        #[structopt(name = "ID")]
        id: i32,

        /// provide name update
        #[structopt(name = "NAME")]
        name: String,
    },
    Department {
        /// Provide the department's id to apply updates to. 
        #[structopt(name = "ID")]
        id: i32,

        /// provide name update
        #[structopt(name = "NAME")]
        name: String,
    }
}