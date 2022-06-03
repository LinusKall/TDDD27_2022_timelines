use cynic::{http::SurfExt, MutationBuilder};
use regex::Regex;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use weblog::*;
use yew::functional::*;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use super::gql::query::*;
use super::Route;
use super::UserId;

#[derive(Properties, PartialEq, Clone)]
pub struct Properties {
    pub set_user_id: Callback<(i32, bool)>,
}

#[function_component(Signup)]
pub fn signup(props: &Properties) -> Html {
    let username = use_state(String::default);
    let password = use_state(String::default);
    let email = use_state(String::default);
    let valid_email = use_state(bool::default);
    let validate = Regex::new(r"^[^ ]+@[^ ]+\.[a-ö]{2,6}$").unwrap();
    let user_id = use_context::<UserId>().expect("No context found.");
    let remain_signed_in = use_state(bool::default);
    let node_ref = use_node_ref();

    if let Some(_) = *user_id.borrow() {
        return html! { <Redirect<Route> to={Route::ListView} /> };
    }

    let user_id_request = {
        let set_user_id = props.set_user_id.clone();
        let username = (*username).to_owned();
        let email = (*email).to_owned();
        let hashed_password = (*password).to_owned();
        let remain_signed_in = remain_signed_in.clone();
        let operation = CreateUser::build(CreateUserInput {
            username,
            email,
            hashed_password,
        });
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not send request")
                .data;

            if let Some(CreateUser { create_user }) = data {
                set_user_id.emit((create_user.id, *remain_signed_in));
                return Ok(create_user.id);
            }
            Err("The username or email is taken.")
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

    let email_input = {
        let current_email = email.clone();
        let current_valid_email = valid_email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if validate.is_match(&input.value()) {
                current_valid_email.set(true);
            } else {
                current_valid_email.set(false);
            }
            current_email.set(input.value());
        })
    };

    let signup_button = {
        let user_id_request = user_id_request.clone();
        Callback::from(move |_| {
            user_id_request.run();
        })
    };

    let checkbox_input = {
        let remain_signed_in = remain_signed_in.clone();
        Callback::from(move |event: Event| {
            let checked = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .checked();
            remain_signed_in.set(checked);
        })
    };

    {
        let node_ref = node_ref.clone();
        use_effect_once(move || {
            node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .focus()
                .unwrap();
            || {}
        });
    }

    html! {
        <div class="signup">
            <input id="username-input" oninput={username_input} placeholder="Username" ref={node_ref}/>
            <input id="password-input" oninput={password_input} type="password" placeholder="Password"/>
            <input id="email-input" oninput={email_input} type="email" placeholder="Email"/>
            <form>
                <input id="remain-signed-in" type="checkbox" checked={*remain_signed_in} onchange={checkbox_input}/>
                <label>
                    <p>{"Remain signed in"}</p>
                    <p>{"(will use a cookie)"}</p>
                </label>
            </form>
            {
                if username.len()<4 || password.len()<8 || !*valid_email {
                    html! {<button id="create-account-button" disabled=true>{"Create account"}</button>}
                } else {
                    html! {<button id="create-account-button" onclick={signup_button}>{"Create account"}</button>}
                }
            }

            {
                if let Some(err) = &user_id_request.error {
                    html! {<p style={"color:Tomato;"}>{format!("{}", err)}</p>}
                } else {
                    html! {}
                }
            }
        </div>
    }
}
