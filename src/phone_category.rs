
#[derive(Debug, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename="phonecategory", rename_all = "lowercase")]
pub enum PhoneNumberCategory {
    Home,
    Cell,
    Extension
}