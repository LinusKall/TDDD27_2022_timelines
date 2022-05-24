use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;
use super::UserId;
use super::USER_ID_KEY;

#[function_component(AccountInfo)]
pub fn account_info() -> Html {
    let username = use_state(String::default);
    let email = use_state(String::default);

    html! {
        <>
            <h1>{"Account Information"}</h1>
            <div>
                <p>{&*username.clone()}</p>
            </div>
            <div>
                <p>{&*email.clone()}</p>
            </div>
            <div>
                <Link<Route> to={Route::ListView}>
                <button /* onclick = {
                    onclick
                } */>{"Change password"}</button>
                </Link<Route>>
            </div>
            <div>
                <Link<Route> to={Route::ListView}>
                    <button /* onclick = {
                        onclick
                    } */>{"Delete account"}</button>
                </Link<Route>>
            </div>
        </>
    }
}