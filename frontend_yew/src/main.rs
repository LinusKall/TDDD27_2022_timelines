mod backend_test;
mod backend_graphql;
mod yew_tutorial;

use backend_graphql::App;


fn main() {
    yew::start_app::<App>();
}
