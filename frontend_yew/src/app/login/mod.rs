use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;
use super::User;

// Consolelog
use weblog::*;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        let current_password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.name() == "username" {
                current_username.set(input.value());
                console_log!("Username: ", input.value());
            } else if input.name() == "password" {
                current_password.set(input.value());
                console_log!("Password: ", input.value());
            } else {
                console_error!("Should be impossible to get here");
            }
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <>
            <div>
                <input name="username" oninput = {oninput.clone()} placeholder="Username"/>
            </div>
            <div>
                <input name="password" {oninput} type="password" placeholder="Password"/>
            </div>
            <Link<Route> to={Route::ListView}> <button onclick = {onclick.clone()} disabled={username.len()<4 || password.len()<8}>{"Log in"}</button></Link<Route>>
            <Link<Route> to={Route::Signup}> <button {onclick}>{"Sign up"}</button></Link<Route>>
        </>
    }
}
