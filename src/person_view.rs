use crate::Phone;
use serde::{Serialize, Deserialize};

/// entry in the personview view
#[derive(Serialize, Deserialize, Debug )]
pub struct PersonView {
    person_id: i32,
    first: String,
    last: String,
    fullname: String,
    login: String,
    phones: Vec<Phone>
}
