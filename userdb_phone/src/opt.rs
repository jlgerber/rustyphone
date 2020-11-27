use structopt::StructOpt;

use userdb_core::PhoneCategory;
use userdb_core::Location;
use userdb_core::NumberString;

//-------------------------
// Structopt Structures
//-------------------------

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



/// Subcommands
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
//
// CREATE
//
/// Create Subcommands
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

//
// READ
//
/// Read Subcommands
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
//
// UPDATE
//
/// Update Subcommands
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
//
// DELETE
//
/// Delete Subcommands
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