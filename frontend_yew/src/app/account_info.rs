use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use web_sys::HtmlButtonElement;
use gloo_storage::LocalStorage;
use weblog::*;

use super::Route;
use super::UserId;
use super::USER_ID_KEY;

#[function_component(AccountInfo)]
pub fn account_info() -> Html {
    let user_id = use_context::<UserId>().expect("No context found.");
    let username = use_state(String::default);
    let password = use_state(String::default);
    let email = use_state(String::default);
    
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
            user_id.delete();
        })
    };
    
    let matches = false;
    let oninput = {
        let current_password = password.clone();
        let matches = matches.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input == current_password {
                matches.set(true);
            } else {
                matches.set(false);
            }
        })
    };

    html! {
        <>
            <h1>{"Account Information"}</h1>
            <div>
                <p>{&*username.clone()}</p>
            </div>
            <div>
                <p>{&*email.clone()}</p>
            </div>
            <button name={"del_acc"} onclick = {
                onclick_delete_ready
            } hidden={
                *delete_ready
            }>{"Delete account"}</button>
            <Link<Route> to={Route::Login}>
                <button name={"del"} onclick = {
                    onclick_delete
                } disabled={
                    (username.len()<4 ||
                    password.len()<8) &&
                    matches
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