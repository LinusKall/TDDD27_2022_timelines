use cynic::{http::SurfExt, QueryBuilder};
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
struct GetUserIdArguments {
    username: String,
    password: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserIdArguments"
)]
struct GetUserId {
    #[arguments(username = &args.username, password = &args.password)]
    get_user_id: Option<i32>,
}

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub set_user_id: Callback<i32>,
}

#[function_component(Login)]
pub fn login(props: &Properties) -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let user_id = use_context::<UserId>().expect("No context found.");

    if let Some(_) = *user_id.borrow() {
        return html! { <Redirect<Route> to={Route::ListView} /> };
    }

    let user_id_request = {
        let set_user_id = props.set_user_id.clone();
        let username = (*username).to_owned();
        let password = (*password).to_owned();
        let operation = GetUserId::build(GetUserIdArguments { username, password });
        use_async(async move {
            let data = surf::post("http://localhost/api/graphql")
                .run_graphql(operation)
                .await
                .expect("Could not send request")
                .data
                .unwrap();

            if let Some(id) = data.get_user_id {
                set_user_id.emit(id);
                return Ok(id);
            }
            Err("Could not fetch user ID.")
        })
    };

    let onclick = {
        let user_id_request = user_id_request.clone();
        Callback::from(move |_| {
            user_id_request.run();
        })
    };

    let username_input = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let password_input = {
        let current_password = password.clone();
        
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_password.set(input.value());
        })
    };

    html! {
        <>
            <Link<Route> to={Route::Signup}>
                <button onclick={
                    onclick.clone()
                }>{"Sign up"}</button>
            </Link<Route>>
            <div>
                <input oninput={username_input} placeholder="Username"/>
            </div>
            <div>
                <input oninput={password_input} type="password" placeholder="Password"/>
            </div>
            <Link<Route> to={Route::ListView}>
                <button onclick = {
                    onclick
                } disabled={
                    username.len()<4 ||
                    password.len()<8
                }>{"Log in"}</button>
            </Link<Route>>
        </>
    }
}
