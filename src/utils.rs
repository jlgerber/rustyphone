
use crate::Person;
use sqlx::FromRow;

/// A little helper function to build up the WHERE clause in 
/// a SELECT query.
pub fn where_joiner(cnt: u8) -> &'static str {
    // 1 is the lowest value that we should encounter, since
    // the index is 1-based.
    if cnt == 1 {
        "WHERE"
    } else {
        "AND"
    }
}

// just a way of extracting the json. We need to be able to implement
// FromRow on something. (unless serde_json::Value has it implemented)
#[derive(FromRow, Debug)]  
pub struct JsonAdapter {
    pub inner: serde_json::Value
}
