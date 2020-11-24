mod utils;
mod constants;
pub mod create;
mod location;
mod mode;
mod person;
mod person_select;
mod person_view;
mod phone;
mod phone_row;
pub mod read;
mod traits;

pub use constants::*;
pub use location::Location;
pub use mode::Mode;
pub use person::Person;
pub use person_select::PersonSelect;
pub use person_view::PersonView;
pub use phone::Phone;
pub use phone_row::PhoneRow;
pub use traits::Selectable;
pub use utils::{print_person};

pub mod prelude {
    pub use super::{Selectable};
}