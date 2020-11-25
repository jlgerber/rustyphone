
mod utils;
mod constants;
pub mod create;
mod errors;
mod location;
mod mode;
mod person;
mod person_select;
mod person_view;
mod phone;
mod phone_category;
mod phone_number;
mod query_mode;
mod phone_row;
pub mod read;
mod traits;

pub use constants::*;
pub use errors::{PhoneError, PhoneErrorKind};
pub use location::Location;
pub use mode::Mode;
pub use person::Person;
pub use person_select::PersonSelect;
pub use person_view::PersonView;
pub use phone::Phone;
pub use phone_category::PhoneCategory;
pub use phone_number::PhoneNumber;
pub use query_mode::QueryMode;
pub use phone_row::PhoneRow;
pub use traits::{Queryable, Selectable, ToStaticStr };
pub use utils::{JsonAdapter, print_person, where_joiner};

pub mod prelude {
    pub use super::{Queryable, Selectable, ToStaticStr};
}