//! The sqlx mode to use. Not Used

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Query,
    FetchAll,
    Json
}