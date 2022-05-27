use cynic::{http::SurfExt, QueryBuilder};
// use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use weblog::*;

use super::Route;
use super::UserId;
use super::gql::query::*;

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub set_user_id: Callback<i32>,
}

#[function_component(Login)]
pub fn login(props: &Properties) -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let user_id = use_context::<UserId>().expect("No context found.");
    let clear_input = use_state(|| bool::default());
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    
    if let Some(_) = *user_id.borrow() {
        return html! { <Redirect<Route> to={Route::ListView} /> };
    }

    let user_id_request = {
        let set_user_id = props.set_user_id.clone();
        let username = (*username).to_owned();
        let password = (*password).to_owned();
        let operation = GetUserId::build(GetUserIdArguments { username, password });
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
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

    let login_click = {
        let user_id_request = user_id_request.clone();
        let clear_input = clear_input.clone();
        Callback::from(move |_| {
            user_id_request.run();
            clear_input.set(true);
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
        let clear_input = clear_input.clone();
        Callback::from(move |e: InputEvent| {
            clear_input.set(false);
            let input: HtmlInputElement = e.target_unchecked_into();
            current_password.set(input.value());
        })
    };

    let enter_input = {
        let user_id_request = user_id_request.clone();
        let clear_input = clear_input.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                user_id_request.run();
                clear_input.set(true);
            }
        })
    };


    let first_render = use_state(|| true);
    {
        let first_render = first_render.clone();
        let username_ref = username_ref.clone(); 
        let password_ref = password_ref.clone();
        use_effect(move || {
            if *first_render {
                username_ref.cast::<HtmlInputElement>().unwrap().focus().unwrap();
                first_render.set(false);
            }
            if *clear_input {
                password_ref.cast::<HtmlInputElement>().unwrap().focus().unwrap();
                password_ref.cast::<HtmlInputElement>().unwrap().set_value("");
            }
            || {}
        });
    }

    html! {
        <>
            <div>
                <input oninput={username_input} onkeypress={enter_input.clone()} ref={username_ref} placeholder="Username"/>
            </div>
            <div>
                <input oninput={password_input} onkeypress={enter_input} ref={password_ref} type="password" placeholder="Password"/>
            </div>
            <button onclick={
                login_click
            } disabled={
                username.len()<4 ||
                password.len()<8
            }>{"Log in"}</button>
            {
                if user_id_request.error.is_some() {
                    html! {<p style={"color:Tomato;"}>{"Wrong username or password. Try again."}</p>}
                } else {
                    html! {}
                }
            }
        </>
    }
}