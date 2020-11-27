use sqlx::FromRow;
use crate::prelude::*;
use crate::NumberString;
use crate::Location;
use crate::PhoneCategory;

pub mod id {
    use super::*;
    /// A datastructure intended to be used to capture the 
    /// changes requested for a specific phone given its id in the database
    #[derive(Debug)]
    pub struct PhoneUpdate {
        pub id: i32,
        pub number: Option<NumberString>,
        pub category: Option<PhoneCategory>,
        pub location: Option<Location>,
    }

    impl Updateable for PhoneUpdate {
        fn is_empty(&self) -> bool {
            self.number.is_none() && 
            self.category.is_none() && 
            self.location.is_none()
        }

        fn update(&self, table: &str) -> String {
            let mut retval = r"
            UPDATE 
                ".to_owned() + table;
            // count starts at 1, but we are already using $1 in the
            // where clause. So our first variable will be $2
            let mut cnt = 2;
            let mut set = "\nSET\n\t";
            if self.number.is_some() {
                retval = format!("{}\n\t{}number = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.category.is_some() {
                retval = format!("{}{}category = ${}",retval,set,cnt);
                set = ",";
                cnt +=1;
            }
            if self.location.is_some() {
                retval = format!("{}{}location = ${}",retval,set,cnt);
                //set = ",";
                //cnt +=1;
            }
           
            retval += "\nWHERE\n\tid = $1\nRETURNING id";
            retval
        }
    }

    impl PhoneUpdate {
        /// Create a new instance of the PhoneUpdate struct with the 
        /// provided phone id
        pub fn new(id: i32) -> Self {
            Self {
                id,
                number: None,
                category: None,
                location: None,
            }
        }
        /// Optionally set the number
        pub fn number(mut self, number: Option<NumberString>) -> Self {
            self.number = number;
            self
        }
        /// Optionally set the category
        pub fn category(mut self, category: Option<PhoneCategory>) -> Self {
            self.category = category;
            self
        }
        /// Optionally set the last name
        pub fn location(mut self, location: Option<Location>) -> Self {
            self.location = location;
            self
        }
    }


    #[derive(FromRow)]
    struct Rval {
        id: Option<i32>
    }

    /// Update a person, based on the supplied values.
    pub async fn update(pool: &sqlx::PgPool, inputs: PhoneUpdate) -> Result<Option<i32>, sqlx::Error> {  

        let update_statement = inputs.update("phone");
        //println!("{}", &update_statement);
        let PhoneUpdate{id, number, category, location} = inputs;
        let mut update= sqlx::query_as(&update_statement).bind(id);
        
        if number.is_some() {
            update = update.bind(number.unwrap().to_string());
        }
        if category.is_some() {
            update = update.bind(category.unwrap());
        }
        if location.is_some() {
            update = update.bind(location.unwrap());
        }
        let Rval{id} = update.fetch_one(pool).await?;
        Ok(id)
    }
}
