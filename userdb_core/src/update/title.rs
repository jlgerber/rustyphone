use sqlx::FromRow;
use crate::prelude::*;


pub mod id {
    use super::*;
    /// A datastructure intended to be used to capture the 
    /// changes requested for a specific phone given its id in the database
    #[derive(Debug)]
    pub struct TitleUpdate {
        pub id: i32,
        pub name: String,
    }

    impl Updateable for TitleUpdate {
        /// This will always return false since TitleUpdate does not contain any Option wrapped
        /// fields.
        fn is_empty(&self) -> bool {
            false
        }

        fn update(&self) -> String {
            //let  retval = "UPDATE\n\ttitle\n\tSET\n\t\tname = $2\nWHERE\n\tid = $1\nRETURNING id".to_owned() ;
            let retval = r"
            UPDATE
                title
            SET
                name = $2
            WHERE
                id = $1
            RETURNING id".to_owned() ;       
            retval
        }
    }

    impl TitleUpdate {
        /// Create a new instance of the TitleUpdate struct with the 
        /// provided phone id
        pub fn new<I: Into<String>>(id: i32, name:I) -> Self {
            Self {
                id,
               name: name.into(),
            }
        }
       
    }


    #[derive(FromRow)]
    struct Rval {
        id: Option<i32>
    }

    /// Update a title, based on the supplied values.
    pub async fn update(pool: &sqlx::PgPool, inputs: TitleUpdate) -> Result<Option<i32>, sqlx::Error> {  

        let update_statement = inputs.update();

        let TitleUpdate{id, name} = inputs;
        let update = sqlx::query_as(&update_statement)
                                .bind(id)
                                .bind(name);
        
        let Rval{id} = update.fetch_one(pool).await?;
        Ok(id)
    }
}


pub mod name {
    use super::*;
    /// A datastructure intended to be used to capture the 
    /// changes requested for a specific phone given its name in the database
    #[derive(Debug)]
    pub struct TitleUpdate {
        pub name: String,
        pub new_name: String,
    }

    impl Updateable for TitleUpdate {
        fn is_empty(&self) -> bool {
            false
        }

        fn update(&self) -> String {
            let  retval = r"
            UPDATE
                title
            SET
                name = $2
            WHERE
                id = ( SELECT 
                            id 
                       FROM 
                            title 
                       WHERE name = $1)
            RETURNING id".to_owned() ;
                          
            retval
        }
    }

    impl TitleUpdate {
        /// Create a new instance of the TitleUpdate struct with the 
        /// provided phone id
        pub fn new<I, J>(name: I, new_name: J) -> Self 
        where 
            I: Into<String>,
            J: Into<String>
        {
            Self {
                name: name.into(),
               new_name: new_name.into(),
            }
        }
    }


    #[derive(FromRow)]
    struct Rval {
        id: Option<i32>
    }

    /// Update a title, based on the supplied values.
    pub async fn update(pool: &sqlx::PgPool, inputs: TitleUpdate) -> Result<Option<i32>, sqlx::Error> {  

        let update_statement = inputs.update();

        let TitleUpdate{name, new_name} = inputs;
        let update = sqlx::query_as(&update_statement)
                                .bind(name)
                                .bind(new_name);
        
        let Rval{id} = update.fetch_one(pool).await?;
        Ok(id)
    }
}
