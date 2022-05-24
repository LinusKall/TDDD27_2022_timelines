use gloo::storage::LocalStorage;
use crate::app::web_sys::HtmlButtonElement;
use gloo_storage::Storage;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use weblog::*;

use super::Route;
use super::UserId;
use super::USER_ID_KEY;

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let user_id = use_context::<UserId>().expect("No context found.");
    let username = use_state(String::default);
    let current_route = use_route::<Route>().unwrap();
    
    let mut login_button = true;
    let mut signup_button = true;
    let mut listview_button = true;

    match current_route {
        Route::Login    => {login_button = false; signup_button = false; listview_button = false; },
        Route::Signup   => {login_button = false; signup_button = false; listview_button = false; },
        Route::ListView => {login_button = true;  signup_button = true;  listview_button = false; },
        Route::NotFound => {login_button = false; signup_button = false; listview_button = false; },
    }

    let onclick = {
        let user_id = user_id.clone();
        Callback::from(move |e: MouseEvent| {
            let elem: HtmlButtonElement = e.target_unchecked_into();
            match &elem.name()[..] {
                "logout"       => {
                    *user_id.borrow_mut() = None;
                    LocalStorage::delete(USER_ID_KEY);
                },
                "account_info" => {},
                "listview"     => {},
                _ => {},
            }
        })
    };

    html! {
        <>
            <Link<Route> to={Route::Login}> <button name={"logout"} onclick={onclick.clone()} hidden={!login_button}>{"Log out"}</button></Link<Route>>
            <Link<Route> to={Route::Signup}> <button name={"account_info"} onclick={onclick.clone()} hidden={!signup_button}>{"Account information"}</button></Link<Route>>
            <Link<Route> to={Route::ListView}> <button name={"listview"} onclick={onclick} hidden={!listview_button}>{"Timelines"}</button></Link<Route>>
        </>
    }
}