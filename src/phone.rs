use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Phone {
    phone_id: i32,
    number: String,
    category: String,
    location: String
} 