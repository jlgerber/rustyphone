use crate::Phone;
use serde::{Serialize, Deserialize};

/// entry in the personview view
#[derive(Serialize, Deserialize, Debug )]
pub struct PersonView {
    pub person_id: i32,
    pub first: String,
    pub last: String,
    pub fullname: String,
    pub login: String,
    pub department: String,
    pub title: String,
    pub phones: Option<Vec<Phone>>
}
