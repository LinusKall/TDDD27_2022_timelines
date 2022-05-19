use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

// Consolelog
use weblog::*;

use regex::Regex;

#[function_component(Signup)]
pub fn signup() -> Html { 
    let username = use_state(String::default);
    let password = use_state(String::default);
    let email    = use_state(String::default);
    let valid_email = use_state(bool::default);
    let user = use_context::<User>().expect("No context found.");
    
    let validate = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,6}$").unwrap();

    let oninput = {
        let current_username = username.clone();
        let current_password = password.clone();
        let current_email    = email.clone();
        let current_valid_email = valid_email.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.name() == "username" {
                current_username.set(input.value());
            } else if input.name() == "password" {
                current_password.set(input.value());
            } else if input.name() == "email" {
                if validate.is_match(&input.value()) {
                    current_valid_email.set(true);
                } else {
                    current_valid_email.set(false);
                }
                current_email.set(input.value());
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
            <form>
                <div>
                    <input name="username" oninput = {oninput.clone()} placeholder="Username"/>
                </div>
                <div>
                    <input name="password" oninput = {oninput.clone()} type="password" placeholder="Password"/>
                </div>
                <div>
                    <input name="email" {oninput} type="email" id="email" placeholder="Email"/>
                </div>
                <Link<Route> to={Route::ListView}> <button onclick = {onclick.clone()} disabled={username.len()<4 || password.len()<8 || !*valid_email}>{"Create account"}</button></Link<Route>>
            </form>
        </>
    }
}