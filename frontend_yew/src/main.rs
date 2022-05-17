mod app;
mod backend_graphql;
mod backend_graphql_userdata;
mod backend_test;
mod yew_tutorial;

use app::App;

fn main() {
    yew::start_app::<App>();
}
