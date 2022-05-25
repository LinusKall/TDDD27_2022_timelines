use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use web_sys::HtmlButtonElement;
use gloo_storage::LocalStorage;
use yew_hooks::prelude::*;
use cynic::{QueryBuilder, http::SurfExt};
use std::ops::Deref;
use weblog::*;

use super::Route;
use super::UserId;
use super::USER_ID_KEY;

mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "UserInfo",
)]
struct UserInfo {
    id: i32,
    username: String,
    email: String,
}

#[derive(cynic::FragmentArguments)]
struct GetUserInfoArgs {
    user_id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserInfoArgs"
)]
struct GetUserInfo {
    #[arguments(user_id = &args.user_id)]
    get_user_info: UserInfo,
}


#[function_component(AccountInfo)]
pub fn account_info() -> Html {
    let user_id = use_context::<UserId>().expect("No context found.");
    let password = use_state(String::default);
    let email = use_state(String::default);
    let first_render = use_state(|| true);

    let user_info = {
        let user_id = user_id.clone();
        let operation = GetUserInfo::build(GetUserInfoArgs { user_id: user_id.borrow_mut().deref().unwrap() });
        use_async(async move {
            let data = surf::post("http://localhost/api/graphql")
                .run_graphql(operation)
                .await
                .expect("Could not send request")
                .data;

            console_log!(format!("USERID {:?}", user_id.borrow_mut().deref()));
            if let Some(user_info) = data {
                console_log!(format!("USERINFO {:?}", user_info));
                return Ok(user_info.get_user_info);
            }
            console_log!("Error");
            Err("Could not fetch userinfo.")
        })
    };

    {
        let user_info = user_info.clone();
        use_effect(move || {
            if *first_render {
                console_log!("FIRST RENDER");
                user_info.run();
                first_render.set(false);
            } else {
                console_log!("NOT FIRST RENDER");
            }
            || {}
        });
    }
    
    let delete_ready = use_state(bool::default);
    let onclick_delete_ready = {
        let user_id = user_id.clone();
        let delete_ready = delete_ready.clone();
        Callback::from(move |_: MouseEvent| {
            delete_ready.set(true);
        })
    };

    
    let onclick_delete = {
        let user_id = user_id.clone();
        Callback::from(move |_: MouseEvent| {
            //user_id.deleteUser();
        })
    };
    
    let matches = use_state(bool::default);
    let oninput = {
        let current_password = password.clone();
        let matches = matches.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            /* if input == current_password {
                matches.set(true);
            } else {
                matches.set(false);
            } */
        })
    };

    html! {
        <>
            <h1>{"Account Information"}</h1>
            <div> 
                {
                    /* if user_info.loading {
                        html! {<p>{"loading"}</p>}
                    } else { */
                    if let Some(data) = &user_info.data {
                        html! {<p>{format!("Username: {}", data.username)}</p>}
                    } else {
                        html! {<p>{"loading"}</p>}
                    }
                }
            </div>
          /*   <div>
                {
                    if user_info.loading {
                        html! {<p>{"loading"}</p>}
                    } else {
                        html! {<p>{format!("Email: {}", user_info.data.clone().unwrap().email)}</p>}
                    }
                }
            </div> */
            <button name={"del_acc"} onclick = {
                onclick_delete_ready
            } hidden={
                *delete_ready
            }>{"Delete account"}</button>
            <Link<Route> to={Route::Login}>
                <button name={"del"} onclick = {
                    onclick_delete
                } disabled={
                    /* (username.len()<4 ||
                    password.len()<8) && */
                    *matches
                } hidden={
                    !*delete_ready
                }>{"Input your password and press this if you are sure you want to delete your account"}</button>
            </Link<Route>>
            <div>
                <input {oninput} hidden={!*delete_ready} type="password" placeholder="Password"/>
            </div>
        </>
    }
}