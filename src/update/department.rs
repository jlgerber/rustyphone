use sqlx::FromRow;
use crate::prelude::*;


pub mod id {
    use super::*;
    /// A datastructure intended to be used to capture the 
    /// changes requested for a specific phone given its id in the database
    #[derive(Debug)]
    pub struct DepartmentUpdate {
        pub id: i32,
        pub name: String,
    }

    impl Updateable for DepartmentUpdate {
        fn is_empty(&self) -> bool {
            false
        }

        fn update(&self) -> String {
            //let  retval = "UPDATE\n\tdepartment\n\tSET\n\t\tname = $2\nWHERE\n\tid = $1\nRETURNING id".to_owned() ;
            let retval = r"
            UPDATE
                department
            SET
                name = $2
            WHERE
                id = $1
            RETURNING id
            ".to_string();          
            retval
        }
    }

    impl DepartmentUpdate {
        /// Create a new instance of the DepartmentUpdate struct with the 
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

    /// Update a department, based on the supplied values.
    pub async fn update(pool: &sqlx::PgPool, inputs: DepartmentUpdate) -> Result<Option<i32>, sqlx::Error> {  

        let update_statement = inputs.update();

        let DepartmentUpdate{id, name} = inputs;
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
    /// changes requested for a specific phone given its id in the database
    #[derive(Debug)]
    pub struct DepartmentUpdate {
        pub name: String,
        pub new_name: String,
    }

    impl Updateable for DepartmentUpdate {
        /// For DepartmentUpdate this will always be false, since
        /// DepartmentUpdate does not contain any Option wrapped values.
        fn is_empty(&self) -> bool {
            false
        }

        fn update(&self) -> String {
            let  retval = r"
            UPDATE
                department
            SET
                name = $2
            WHERE
                id = ( SELECT 
                            id 
                       FROM 
                            department 
                       WHERE name = $1)
            RETURNING id".to_owned() ;
                          
            retval
        }
    }

    impl DepartmentUpdate {
        /// Create a new instance of the DepartmentUpdate struct with the 
        /// provided current department name
        ///
        /// # Example
        /// ```rust
        /// let dept_update = DepartmentUpdate::new("Mt Employees", "Montreal Artists");
        /// ```
        pub fn new<I, J>(name: I, new_name:J) -> Self 
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

    /// Update a department, based on the supplied values.
    pub async fn update(pool: &sqlx::PgPool, inputs: DepartmentUpdate) -> Result<Option<i32>, sqlx::Error> {  

        let update_statement = inputs.update();

        let DepartmentUpdate{name, new_name} = inputs;

        let update = sqlx::query_as(&update_statement)
                                .bind(name)
                                .bind(new_name);
        
        let Rval{id} = update.fetch_one(pool).await?;
        Ok(id)
    }
}
