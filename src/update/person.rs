//use crate::errors::PhoneError;
use sqlx::FromRow;
use crate::prelude::*;

pub mod id {
    use super::*;
    /// A datastructure intended to be used to capture the 
    /// change requests
    #[derive(Debug)]
    pub struct PersonUpdate {
        pub id: i32,
        pub login: Option<String>,
        pub first: Option<String>,
        pub last: Option<String>,
        pub department: Option<i32>,
        pub title: Option<i32>
    }

    impl Updateable for PersonUpdate {
        fn is_empty(&self) -> bool {
            self.login.is_none() && 
            self.first.is_none() && 
            self.last.is_none() && 
            self.department.is_none() &&
            self.title.is_none()
        }

        fn update(&self, table: &str) -> String {
            let mut retval = r"
            UPDATE 
                ".to_owned() + table;
            // count starts at 1, but we are already using $1 in the
            // where clause. So our first variable will be $2
            let mut cnt = 2;
            let mut set = "\nSET\n\t";
            if self.login.is_some() {
                retval = format!("{}\n\t{}login = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.first.is_some() {
                retval = format!("{}{}first = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.last.is_some() {
                retval = format!("{}{}last = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.department.is_some() {
                retval = format!("{}{}department_id = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.title.is_some() {
                retval = format!("{}{}title_id = ${}",retval,set,cnt);
                //set = ",";
                //cnt +=1;
            }
            retval += "\nWHERE\n\tid = $1\nRETURNING id";
            retval
        }
    }

    impl PersonUpdate {
        pub fn new(id: i32) -> Self {
            Self {
                id,
                login: None,
                first: None,
                last: None,
                department: None,
                title: None,
            }
        }
        /// Set the login
        pub fn login(mut self, login: Option<String>) -> Self {
            self.login = login;
            self
        }
        /// Set the first name
        pub fn first(mut self, first: Option<String>) -> Self {
            self.first = first;
            self
        }
        /// Set the last name
        pub fn last(mut self, last: Option<String>) -> Self {
            self.last = last;
            self
        }
        /// Set the department id
        pub fn department(mut self, department: Option<i32>) -> Self {
            self.department = department;
            self
        }
        /// Set the title id
        pub fn title(mut self, title: Option<i32>) -> Self {
            self.title = title;
            self
        }
    }


    #[derive(FromRow)]
    struct Rval {
        id: Option<i32>
    }

    /// Update a person, based on the supplied values.
    pub async fn update(pool: &sqlx::PgPool, inputs: PersonUpdate) -> Result<Option<i32>, sqlx::Error> {  

        let update_statement = inputs.update("person");
        //println!("{}", &update_statement);
        let PersonUpdate{id, login, first, last, department, title} = inputs;
        let mut update= sqlx::query_as(&update_statement).bind(id);
        
        if login.is_some() {
            update = update.bind(login.unwrap());
        }
        if first.is_some() {
            update = update.bind(first.unwrap());
        }
        if last.is_some() {
            update = update.bind(last.unwrap());
        }
        if department.is_some() {
            update = update.bind(department.unwrap());
        }
        if title.is_some() {
            update = update.bind(title.unwrap());
        }
        let Rval{id} = update.fetch_one(pool).await?;
        Ok(id)
    }
}


pub mod login {
    use super::*;
    /// A datastructure intended to be used to capture the 
    /// change requests
    #[derive(Debug)]
    pub struct PersonUpdate {
        pub current_login: String,
        pub login: Option<String>,
        pub first: Option<String>,
        pub last: Option<String>,
        pub department: Option<i32>,
        pub title: Option<i32>
    }

    impl Updateable for PersonUpdate {
        fn is_empty(&self) -> bool {
            self.login.is_none() && 
            self.first.is_none() && 
            self.last.is_none() && 
            self.department.is_none() &&
            self.title.is_none()
        }

        fn update(&self, table: &str) -> String {
            let mut retval = r"
            UPDATE 
                ".to_owned() + table;
            // count starts at 1, but we are already using $1 in the
            // where clause. So our first variable will be $2
            let mut cnt = 2;
            let mut set = "\nSET\n\t";
            if self.login.is_some() {
                retval = format!("{}\n\t{}login = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.first.is_some() {
                retval = format!("{}{}first = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.last.is_some() {
                retval = format!("{}{}last = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.department.is_some() {
                retval = format!("{}{}department_id = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.title.is_some() {
                retval = format!("{}{}title_id = ${}",retval,set,cnt);
                //set = ",";
                //cnt +=1;
            }
            retval += "\nWHERE\n\tid = (\n\tSELECT\n\t\tperson.id\n\tFROM\n\t\tperson\n\tWHERE\n\t\tperson.login = $1\n\t)\nRETURNING id";
            retval
        }
    }

    impl PersonUpdate {
        pub fn new<I: Into<String>>(current_login: I) -> Self {
            Self {
                current_login: current_login.into(),
                login: None,
                first: None,
                last: None,
                department: None,
                title: None,
            }
        }
        /// Set the login
        pub fn login(mut self, login: Option<String>) -> Self {
            self.login = login;
            self
        }
        /// Set the first name
        pub fn first(mut self, first: Option<String>) -> Self {
            self.first = first;
            self
        }
        /// Set the last name
        pub fn last(mut self, last: Option<String>) -> Self {
            self.last = last;
            self
        }
        /// Set the department id
        pub fn department(mut self, department: Option<i32>) -> Self {
            self.department = department;
            self
        }
        /// Set the title id
        pub fn title(mut self, title: Option<i32>) -> Self {
            self.title = title;
            self
        }
    }


    #[derive(FromRow)]
    struct Rval {
        id: Option<i32>
    }

    /// Update a person, based on the supplied values.
    pub async fn update(pool: &sqlx::PgPool, inputs: PersonUpdate) -> Result<Option<i32>, sqlx::Error> {  

        let update_statement = inputs.update("person");
        //println!("{}", &update_statement);
        let PersonUpdate{current_login, login, first, last, department, title} = inputs;
        let mut update= sqlx::query_as(&update_statement).bind(current_login);
        
        if login.is_some() {
            update = update.bind(login.unwrap());
        }
        if first.is_some() {
            update = update.bind(first.unwrap());
        }
        if last.is_some() {
            update = update.bind(last.unwrap());
        }
        if department.is_some() {
            update = update.bind(department.unwrap());
        }
        if title.is_some() {
            update = update.bind(title.unwrap());
        }
        let Rval{id} = update.fetch_one(pool).await?;
        Ok(id)
    }
}