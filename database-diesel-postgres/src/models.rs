use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum)]
#[PgType = "clearance"]
pub enum Clearance {
    Owner,
    Moderator,
    Subscriber,
}
