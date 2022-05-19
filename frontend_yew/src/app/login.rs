use cynic::{http::SurfExt, QueryBuilder};
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
// use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use super::Route;
use super::UserId;

// Consolelog
use weblog::*;

mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

#[derive(cynic::FragmentArguments)]
struct GetUserdIdArguments {
    username: String,
    password: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserdIdArguments"
)]
struct GetUserdId {
    #[arguments(username = &args.username, password = &args.password)]
    get_user_id: Option<i32>,
}

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let user_id = use_context::<UserId>().expect("No context found.");

    let user_id_request = {
        let username = (*username).to_owned();
        let password = (*password).to_owned();
        let cuid = user_id.clone();
        let operation = GetUserdId::build(GetUserdIdArguments { username, password });
        use_async(async move {
            let data = surf::post("http://localhost/api/graphql")
                .run_graphql(operation)
                .await
                .expect("Could not send request")
                .data
                .unwrap();

            console_log!(format!("{:#?}", &data));
            if let Some(id) = data.get_user_id {
                *cuid.borrow_mut() = Some(id);
                LocalStorage::set("timelines_user_id", id).unwrap();
                return Ok(id);
            }
            Err("Could not fetch user ID.")
        })
    };

    let onclick = {
        let user_id_request = user_id_request.clone();
        Callback::from(move |_| user_id_request.run())
    };

    let oninput = {
        let current_username = username.clone();
        let current_password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.name() == "username" {
                current_username.set(input.value());
                console_log!("Username: ", input.value());
            } else if input.name() == "password" {
                current_password.set(input.value());
                console_log!("Password: ", input.value());
            } else {
                console_error!("Should be impossible to get here");
            }
        })
    };

    html! {
        <>
            <div>
                <input name="username" oninput = {oninput.clone()} placeholder="Username"/>
            </div>
            <div>
                <input name="password" {oninput} type="password" placeholder="Password"/>
            </div>
            <Link<Route> to={Route::ListView}>
                <button onclick = {
                    onclick.clone()
                } disabled={
                    username.len()<4 ||
                    password.len()<8
                }>{"Log in"}</button>
            </Link<Route>>
            <Link<Route> to={Route::Signup}>
                <button {onclick}>{"Sign up"}</button>
            </Link<Route>>
            {
                if user_id_request.loading {
                    html! { "Loading" }
                } else {
                    html! {}
                }
            }
        </>
    }
}
