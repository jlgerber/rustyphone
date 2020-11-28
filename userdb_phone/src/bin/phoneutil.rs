//! phoneutil command
//! 
//! Cli which exposes the full lifecycle for the phonedb.
//! End users should use `phone`. 
//! This command allows one to mutate the database. 
use colored::*;
use structopt::StructOpt;

// internal
use userdb_core::QueryMode;
use userdb_core::read::phone::PhoneQuery;
use userdb_phone::opt::*;
use userdb_phone::process::read::*;
use userdb_phone::process::create::*;
use userdb_phone::process::update::*;
use userdb_phone::process::delete::*;


#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // build options from structopt
    let opt = Opt::from_args();
    match opt {
        Opt{mut name, login, json, title, dept, fullname, cmd: None} => {
            if name.is_none() && fullname.is_some() {
                name = fullname;
            }
            process_read_person(name, login, title, dept, json ).await},
        Opt{cmd: Some(OptSub::Read{sub}), ..} => match sub {
            ReadOpt::Person{mut name, login, title, dept, fullname, json} => {
                if name.is_none() && fullname.is_some() {
                    name = fullname;
                }
                process_read_person(name, login, title, dept, json).await
            },
            ReadOpt::Title{json} => process_read_title(json).await,
            ReadOpt::Department{json} => process_read_department(json).await,
            ReadOpt::Phone{id, number, category, location, json} => {
                
                let query = PhoneQuery::new()
                .id(id)
                .number(number)
                .category(category)
                .location(location);
                process_read_phone(query, QueryMode::ILike, json ).await}
        }
        Opt{cmd: Some(OptSub::Create{sub}), ..} => match sub {
            CreateOpt::Person{first, last, login, department, title} => process_create_person(&first, &last, &login, &department, &title).await,
            CreateOpt::Phone{login, number, category, location} => process_create_phone(&login, &number, &category, &location).await,
            CreateOpt::Title{title} => process_create_title(&title).await,
            CreateOpt::Department{department} => process_create_department(&department).await,
        }
        Opt{cmd: Some(OptSub::Update{sub}), ..} => match sub {
            UpdateOpt::Person{ id: Some(id), first, last, login, department, title,..} => process_update_person_by_id(id, first, last, login, department, title).await,
            UpdateOpt::Person{ from_login: Some(from_login), first, last, login, department, title,..} => process_update_person_by_login(from_login, first, last, login, department, title).await,
            UpdateOpt::Person{..} => {
                eprintln!("\n\t{} Must supply either --id or --name.", "Error:".bright_red());
                std::process::exit(1);
            },
            UpdateOpt::Phone{id, number, category, location} => process_update_phone_by_id(id, number, category, location).await,
            UpdateOpt::Title{id, name} => process_update_title_by_id(id, name).await,
            UpdateOpt::Department{id, name} => process_update_dept_by_id(id, name).await,
        }
        Opt{cmd: Some(OptSub::Delete{sub}), ..} => match sub {
            DeleteOpt::Phone{id: Some(id),..} => process_delete_phone_by_id(id).await,
            DeleteOpt::Phone{
                login: Some(login), 
                number: Some(number), 
                category:Some(category), 
                location: Some(location),..} => process_delete_phone(&login, &number, &category, &location).await,
            DeleteOpt::Phone{..} => panic!("should not reach here"),
            DeleteOpt::Department{name: Some(value), ..} => process_delete_dept(&value).await,
            DeleteOpt::Department{id: Some(id),..} => process_delete_dept_by_id(id).await,
            DeleteOpt::Department{..} => {
                eprintln!("\n\t{} Must supply either --id or --name.", "Error:".bright_red());
                std::process::exit(1);
            },

            DeleteOpt::Title{name: Some(value), ..} => process_delete_title(&value).await,
            DeleteOpt::Title{id: Some(id),..} => process_delete_title_by_id(id).await,
            DeleteOpt::Title{..} => {
                eprintln!("\n\t{} Must supply either --id or --name.", "Error:".bright_red());
                std::process::exit(1);
            },

            DeleteOpt::Person{login: Some(value), ..} => process_delete_person(&value).await,
            DeleteOpt::Person{id: Some(id),..} => process_delete_person_by_id(id).await,
            DeleteOpt::Person{..} => {
                eprintln!("\n\t{} Must supply either --id or --login.", "Error:".bright_red());
                std::process::exit(1);
            }
        }
    }
}
