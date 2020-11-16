#[derive(Debug,PartialEq, Eq, sqlx::Type)]
#[sqlx(rename="location", rename_all = "lowercase")]
pub enum Location {
    Portland,
    PlayaVista,
    Vancouver,
    Montreal,
    Hyderabad
}