use diesel_derive_enum::DbEnum;

/*
Do  **NOT**  forget to add "Mapping" at the end of each enum type in src/schema.rs!!
*/

#[derive(Debug, DbEnum)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum Clearance {
    Owner,
    Moderator,
    Subscriber,
}
