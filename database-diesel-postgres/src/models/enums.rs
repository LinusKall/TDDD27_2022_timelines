use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum Clearance {
    Owner,
    Moderator,
    Subscriber,
}

pub mod exports {
    pub use super::Clearance as ClearanceMapping;
}