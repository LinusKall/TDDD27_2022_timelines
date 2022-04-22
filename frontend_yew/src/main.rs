mod app;
mod backend_graphql;
mod backend_test;
mod yew_tutorial;
use app::*;

fn main() {
    yew::start_app::<App>();
}
