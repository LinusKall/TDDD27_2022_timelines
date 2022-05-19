use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

// Consolelog
use weblog::*;

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let user = use_context::<User>().expect("No context found.");
    let username = use_state(String::default);

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <>
            <Link<Route> to={Route::Login}> <button onclick = {onclick.clone()} >{"Log out"}</button></Link<Route>>
            <Link<Route> to={Route::Signup}> <button onclick = {onclick.clone()} >{"Accout information / Sign up"}</button></Link<Route>>
            <Link<Route> to={Route::ListView}> <button onclick = {onclick.clone()} >{"Timelines"}</button></Link<Route>>
            <Link<Route> to={Route::Home}> <button onclick = {onclick.clone()} >{"Home / about"}</button></Link<Route>>
        </>
    }
}