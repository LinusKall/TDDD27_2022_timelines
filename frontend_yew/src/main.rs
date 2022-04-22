mod backend_test;
mod backend_graphql;
mod yew_tutorial;
mod app;
use app::*;

fn main() {
    yew::start_app::<App>();
}
