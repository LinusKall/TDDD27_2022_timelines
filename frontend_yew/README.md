# Build for the backend

1. Make your application.
1. Use `trunk build` in this directory
    * This will place the application in the `dist/` directory.
    * I you don't have `trunk` installed use `cargo install trunk`.
1. Use `cargo run` in parallel crate `backend-***`.
1. Visit `localhost:3000/` (we use the port 3000 for testing).

To test the `src/yew_tutorial` module, use `trunk serve --open --proxy-backend=https://yew.rs/tutorial`.