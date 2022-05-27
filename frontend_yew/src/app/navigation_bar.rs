use gloo::storage::LocalStorage;
use gloo_storage::Storage;
#[allow(unused)]
use weblog::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;
use super::UserId;
use super::USER_ID_KEY;

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let user_id = use_context::<UserId>().expect("No context found.");
    let current_route = use_route::<Route>().unwrap();

    let login_button;
    let signup_button;
    let logout_button;
    let account_info_button;
    let listview_button;

    #[rustfmt::skip]
    match current_route {
        Route::Login       => { login_button = false; signup_button = true;  logout_button = false; account_info_button = false; listview_button = false; },
        Route::Signup      => { login_button = true;  signup_button = false; logout_button = false; account_info_button = false; listview_button = false; },
        Route::ListView    => { login_button = false; signup_button = false; logout_button = true;  account_info_button = true;  listview_button = false; },
        Route::AccountInfo => { login_button = false; signup_button = false; logout_button = false; account_info_button = false; listview_button = true;  },
        Route::NotFound    => { login_button = false; signup_button = false; logout_button = false; account_info_button = false; listview_button = false; },
    }

    let onclick_login = { Callback::from(move |_: MouseEvent| {}) };

    let onclick_signup = { Callback::from(move |_: MouseEvent| {}) };

    let onclick_logout = {
        let user_id = user_id.clone();
        Callback::from(move |_: MouseEvent| {
            *user_id.borrow_mut() = None;
            LocalStorage::delete(USER_ID_KEY);
        })
    };

    let onclick_account_info = { Callback::from(move |_: MouseEvent| {}) };

    let onclick_listview = { Callback::from(move |_: MouseEvent| {}) };

    html! {
        <div class="navbar">
            {
                match login_button {
                    true => html! {
                        <Link<Route> to={Route::Login}>
                            <button onclick={onclick_login}>{"Log in"}</button>
                        </Link<Route>>
                    },
                    false => html! {},
                }
            }
            {
                match signup_button {
                    true => html! {
                        <Link<Route> to={Route::Signup}>
                            <button onclick={onclick_signup}>{"Sign up"}</button>
                        </Link<Route>>
                    },
                    false => html! {},
                }
            }
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
        </div>
    }
}
