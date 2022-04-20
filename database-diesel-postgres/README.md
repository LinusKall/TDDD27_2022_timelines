# Requirements

1. Install PostgreSQL:
    1. Make sure to add the environment variable ``PQ_LIB_DIR``. Set it to ``PATH/TO/YOUR/PostgreSQL/VERSION/lib``.
    1. Also add these to your PATH variable:
        1. ``PATH/TO/YOUR/PostgreSQL/VERSION/lib``.
        1. ``PATH/TO/YOUR/PostgreSQL/VERSION/bin``.
1. Install ``diesel_cli`` using the command: ``cargo install diesel_cli --no-default-features --features postgres``.

# How this was setup

We used the following guide to get started on our database using Diesel and PostgreSQL: [https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started).

For the database design we used the following example as guidance: [https://github.com/diesel-rs/diesel/tree/master/examples/postgres/advanced-blog-cli](https://github.com/diesel-rs/diesel/tree/master/examples/postgres/advanced-blog-cli).

Most of the guides from the following are also useful: [https://diesel.rs/guides/](https://diesel.rs/guides/).

# NOTICE: When making enums

To make the enum experience less painless use these two rules:

First, here is an example of how you should define your enum in SQL:

```sql
CREATE TYPE clearance_mapping AS ENUM ('OWNER', 'MODERATOR', 'SUBSCRIBER');

CREATE TABLE timelines_users (
    timeline_id SERIAL NOT NULL REFERENCES timelines (id),
    user_id SERIAL NOT NULL REFERENCES users (id),
    relation clearance_mapping NOT NULL,
    color TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (timeline_id, user_id)
);
```

Second, here is an example of how the corresponding enum should look in Rust:

```rust
/** 
 * The DbEnum derive does the following for us automatically: 
 * https://kitsu.me/posts/2020_05_24_custom_types_in_diesel
 * 
 * DbValueStyle describes what "Owner", "Moderator", and "Subscriber" looks like in the SQL.
 * In our SQL we can see that the values are in SCREAMIN_SNAKE_CASE.
 * 
 * PgType is the name of the enum exactly as it is defined in the SQL.
 * 
 * DieselType is the name of the type that the Diesel Cli will autmatically generate
 * from the definition in the SQL.
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
```