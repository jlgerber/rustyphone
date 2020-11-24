mod utils;
mod constants;
pub mod create;
mod location;
mod mode;
mod person;
mod person_select;
mod person_view;
mod phone;
mod phone_category;
mod phone_row;
pub mod read;
mod traits;
pub mod errors;
mod query_mode;

pub use constants::*;
pub use location::Location;
pub use mode::Mode;
pub use person::Person;
pub use person_select::PersonSelect;
pub use person_view::PersonView;
pub use phone::Phone;
pub use phone_category::PhoneCategory;
pub use phone_row::PhoneRow;
pub use query_mode::QueryMode;
pub use traits::{Selectable, ToStaticStr, Queryable};
pub use utils::{print_person, where_joiner, JsonAdapter};
pub mod prelude {
    pub use super::{Selectable, ToStaticStr, Queryable};
}