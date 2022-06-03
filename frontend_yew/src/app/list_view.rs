use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use yew::prelude::*;
use yew_router::prelude::*;

#[allow(unused)]
use weblog::*;

use super::timeline_list::*;
use super::Route;
use super::UserId;

#[function_component(ListView)]
pub fn list_view() -> Html {
    let user_id = use_context::<UserId>().expect("No context found.");

    if user_id.borrow().is_none() {
        *user_id.borrow_mut() = match LocalStorage::get("timelines_user_id") {
            Ok(uid) => uid,
            _ => {
                return html! {
                    <Redirect<Route> to={Route::Login} />
                }
            }
        };
    }

    html! {
        <div class="list_view">
            <TimelineList user_id={*user_id.borrow().as_ref().unwrap()}/>
        </div>
    }
}
