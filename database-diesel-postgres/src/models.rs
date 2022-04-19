use diesel_derive_enum::DbEnum;
/* use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

pub mod exports {
    pub use super::ClearanceSQL as Clearance;
}

#[derive(SqlType)]
#[postgres(type_name = "Clearance")]
pub struct ClearanceSQL;

#[derive(Debug, AsExpression, FromSqlRow)]
#[sql_type = "ClearanceSQL"] */
#[derive(Debug, DbEnum, SqlType)]
#[PgType = "clearance"]
pub enum Clearance {
    Owner,
    Moderator,
    Subscriber,
}

/* impl<Db: Backend> ToSql<ClearanceSQL, Db> for Clearance {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            Clearance::Owner => out.write_all(b"owner")?,
            Clearance::Moderator => out.write_all(b"moderator")?,
            Clearance::Subscriber => out.write_all(b"subscriber")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ClearanceSQL, Pg> for Clearance {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"owner" => Ok(Clearance::Owner),
            b"moderator" => Ok(Clearance::Moderator),
            b"subscriber"=> Ok(Clearance::Subscriber),
            _ => Err("Unrecognized clearance".into()),
        }
    }
} */
