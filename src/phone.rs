use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Phone {
    pub phone_id: i32,
    pub number: String,
    pub category: String,
    pub location: String
} 