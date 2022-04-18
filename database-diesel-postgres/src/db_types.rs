use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum)]
pub enum UserRole {
    Owner,
    Moderator,
    Subscriber,
}