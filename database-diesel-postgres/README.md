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