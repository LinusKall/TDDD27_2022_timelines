use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

// Consolelog
use weblog::*;

#[function_component(Signup)]
pub fn signup() -> Html { 
    let username = use_state(String::default);
    let password = use_state(String::default);
    let email    = use_state(String::default);
    let valid_email = use_state(bool::default);
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        let current_password = password.clone();
        let current_email    = email.clone(); // TODO: Actually check that pattern is correct

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.name() == "username" {
                current_username.set(input.value());
                console_log!("Username: ", input.value());
            } else if input.name() == "password" {
                current_password.set(input.value());
                console_log!("Password: ", input.value());
            } else if input.name() == "email" {
                if input.value().contains("@") && input.value().contains(".") {
                    valid_email.set(true);
                    console_log!("valid", *valid_email);
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
            <div>
                <input name="username" oninput = {oninput.clone()} placeholder="Username"/>
            </div>
            <div>
                <input name="password" oninput = {oninput.clone()} type="password" placeholder="Password"/>
            </div>
            <div>
                <input name="email" {oninput} type="email" id="email" placeholder="Email" pattern="[a-z0-9._%+-]+@[a-z0-9.-]+.[a-z]{2,4}$"/>
            </div>
            <Link<Route> to={Route::ListView}> <button onclick = {onclick.clone()} disabled={username.len()<4 || password.len()<8 || !email.contains(".") || !email.contains("@")  }>{"Create account"}</button></Link<Route>>
        </>
    }
}