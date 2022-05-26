use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use yew_hooks::prelude::*;
use cynic::{QueryBuilder, http::SurfExt, MutationBuilder};
use std::ops::Deref;
use weblog::*;

use super::Route;
use super::UserId;
use super::USER_ID_KEY;
use super::gql::query::*;

#[function_component(AccountInfo)]
pub fn account_info() -> Html {
    let user_id = use_context::<UserId>().expect("No context found.");
    let password = use_state(|| String::new());
    let incorrect_password = use_state(bool::default);
    let first_render = use_state(|| true);

    if user_id.borrow().is_none() {
        return html! { <Redirect<Route> to={Route::Login} /> };
    }

    let user_info = {
        let user_id = user_id.clone();
        let operation = GetUserInfo::build(GetUserInfoArgs { user_id: user_id.borrow_mut().deref().unwrap() });
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not send request")
                .data;

            if let Some(user_info) = data {
                return Ok(user_info.get_user_info);
            }
            Err("Could not fetch userinfo.")
        })
    };

    {
        let user_info = user_info.clone();
        let first_render = first_render.clone();
        use_effect(move || {
            if *first_render {
                user_info.run();
                first_render.set(false);
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

    let delete_user = {
        let user_id = user_id.clone();
        let password = password.deref().to_owned();
        let incorrect_password = incorrect_password.clone();
        let first_render = first_render.clone();
        let operation = DeleteUser::build(DeleteUserInput {user_id: user_id.borrow_mut().deref().unwrap(), password: password});
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not send request")
                .data;
            if let Some(delete_user_result) = data {
                *user_id.borrow_mut() = None;
                LocalStorage::delete(USER_ID_KEY);
                first_render.set(true);
                return Ok(delete_user_result.delete_user);
            }
            incorrect_password.set(true);
            Err("Could not delete user.")
        })
    };
    
    let onclick_delete = {
        let delete_user = delete_user.clone();
        Callback::from(move |_: MouseEvent| {
            delete_user.run();
        })
    };
    
    let oninput = {
        let current_password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_password.set(input.value());
        })
    };

    html! {
        <>
            <h2>{"Account Information"}</h2>
            <div> 
                {
                    if let Some(data) = &user_info.data {
                        html! {
                            <>
                                <p>{format!("Username: {}", data.username)}</p>
                                <p>{format!("Email: {}", data.email)}</p>
                            </>
                        }
                    } else {
                        html! {<p>{"Loading"}</p>}
                    }
                }
            </div>
            <button name={"del_acc"} onclick = {
                onclick_delete_ready
            } hidden={
                *delete_ready
            }>{"Delete account"}</button>
            <button name={"del"} onclick = {
                onclick_delete
            } hidden={
                !*delete_ready
            }>{"Input your password and press this if you are sure you want to delete your account"}</button>
            <div>
                <input {oninput} hidden={!*delete_ready} type="password" placeholder="Password"/>
                <p hidden={!*incorrect_password} style={"color:Tomato;"}>{"Wrong password. Try again."}</p>
            </div>
        </>
    }
}