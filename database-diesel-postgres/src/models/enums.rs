use diesel_derive_enum::DbEnum;


/** 
 * The DbEnum derive does the following for us automatically: 
 * https://kitsu.me/posts/2020_05_24_custom_types_in_diesel
 * 
 * DbValueStyle describes what "Owner", "Moderator", and "Subscriber" looks like in the SQL.
 * In our SQL we can see that the values are in SCREAMIN_SNAKE_CASE.
 * 
 * PgType is the name of the enum exactly as it is defined in the SQL.
 * 
 * DieselType is the name of the type that the Diesel Cli will autmatically generate from the definition in the SQL.
 */
#[derive(Debug, DbEnum)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[PgType = "clearance_mapping"]
#[DieselType = "Clearance_mapping"]
pub enum Clearance {
    Owner,
    Moderator,
    Subscriber,
}
