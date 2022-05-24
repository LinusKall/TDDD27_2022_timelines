use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use weblog::*;

use crate::Route;
use crate::User;

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let user = use_context::<User>().expect("No context found.");
    let username = use_state(String::default);
    let current_route = use_route::<Route>().unwrap();
    
    let mut login_button = true;
    let mut signup_button = true;
    let mut listview_button = true;

    match current_route {
        Route::ListView => {login_button = true;  signup_button = true;  listview_button = false; },
        Route::Login    => {login_button = false; signup_button = false; listview_button = false; },
        Route::Signup   => {login_button = false; signup_button = false; listview_button = false; },
        Route::NotFound => {login_button = false; signup_button = false; listview_button = false; },
        Route::Home     => {},
        Route::Secure   => {}
    }

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <>
            <Link<Route> to={Route::Login}> <button onclick={onclick.clone()} hidden={!login_button}>{"Log out"}</button></Link<Route>>
            <Link<Route> to={Route::Signup}> <button onclick={onclick.clone()} hidden={!signup_button}>{"Account information"}</button></Link<Route>>
            <Link<Route> to={Route::ListView}> <button onclick={onclick.clone()} hidden={!listview_button}>{"Timelines"}</button></Link<Route>>
        </>
    }
}