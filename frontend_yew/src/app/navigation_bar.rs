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
    let current_route = use_route::<Route>().unwrap();
    
    let logout_button;
    let account_info_button;
    let listview_button;

    match current_route {
        Route::Login       => {logout_button = false; account_info_button = false; listview_button = false; },
        Route::Signup      => {logout_button = false; account_info_button = false; listview_button = false; },
        Route::ListView    => {logout_button = true;  account_info_button = true;  listview_button = false; },
        Route::AccountInfo => {logout_button = false; account_info_button = false; listview_button = true;  },
        Route::NotFound    => {logout_button = false; account_info_button = false; listview_button = false; },
    }

    let onclick_logout = {
        let user_id = user_id.clone();
        Callback::from(move |_: MouseEvent| {
            *user_id.borrow_mut() = None;
            LocalStorage::delete(USER_ID_KEY);
        })
    };

    let onclick_account_info = {
        Callback::from(move |_: MouseEvent| {})
    };

    let onclick_listview = {
        Callback::from(move |_: MouseEvent| {})
    };

    html! {
        <>
            {
                match logout_button {
                    true => html! {
                        <Link<Route> to={Route::Login}> 
                            <button onclick={onclick_logout}>{"Log out"}</button>
                        </Link<Route>>
                    },
                    false => html! {},
                }
            }
            {
                match account_info_button {
                    true => html! {
                        <Link<Route> to={Route::AccountInfo}> 
                            <button onclick={onclick_account_info} >{"Account information"}</button>
                        </Link<Route>>
                    },
                    false => html! {},
                }
            }
            {
                match listview_button {
                    true => html! {
                        <Link<Route> to={Route::ListView}> 
                            <button onclick={onclick_listview} >{"Timelines"}</button>
                        </Link<Route>>
                    },
                    false => html! {},
                }
            }
            /* <Link<Route> to={Route::Login}> <button onclick={onclick_logout} hidden={!logout_button}>{"Log out"}</button></Link<Route>>
            <Link<Route> to={Route::AccountInfo}> <button onclick={onclick_account_info} hidden={!account_info_button}>{"Account information"}</button></Link<Route>>
            <Link<Route> to={Route::ListView}> <button onclick={onclick_listview} hidden={!listview_button}>{"Timelines"}</button></Link<Route>> */
        </>
    }
}