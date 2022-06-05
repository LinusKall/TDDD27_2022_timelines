mod app;
mod test_apps;

// use test_apps::backend_graphql_userdata::App;
use app::App;

fn main() {
    yew::start_app::<App>();
}
