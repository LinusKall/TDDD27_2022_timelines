# Requirements

1. Install PostgreSQL:
    1. Make sure to add the environment variable ``PQ_LIB_DIR``. Set it to ``PATH/TO/YOUR/POSTGRES/INSTALLATION/lib``.
    1. Also add these to your PATH variable:
        1. ``PATH/TO/YOUR/POSTGRES/INSTALLATION/bin``.
        1. ``PATH/TO/YOUR/POSTGRES/INSTALLATION/lib``.
1. Install ``diesel_cli`` using the command: ``cargo install diesel_cli --no-default-features --features postgres``.

# How this was setup

Look at the following guide to get started on you own database using Diesel: [https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started)