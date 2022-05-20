use super::list_selector::*;
use super::task_info::*;
use super::task_list::*;
use std::ops::Deref;
// use db::api::*;
use crate::app::Route;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
#[allow(unused_imports)]
use weblog::*;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;

use super::UserId;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Timeline {
    pub user: String,
    pub color: (u8, u8, u8),
    pub name: String,
    pub task: String,
}

#[function_component(ListView)]
pub fn list_view() -> Html {
    let timeline_state = use_state(Timeline::default);
    let user_id = use_context::<UserId>().expect("No context found.");

    // LocalStorage::delete("timelines_user_id");

    *user_id.borrow_mut() = match LocalStorage::get("timelines_user_id") {
        Ok(uid) => uid,
        _ => {
            return html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
    };

    let timeline_switch = {
        let timeline_state = timeline_state.clone();
        Callback::from(move |name: String| {
            let mut timeline = timeline_state.deref().clone();
            timeline.name = name;
            timeline_state.set(timeline);
        })
    };
    let task_switch = {
        let timeline_state = timeline_state.clone();
        Callback::from(move |task: String| {
            let mut timeline = timeline_state.deref().clone();
            timeline.task = task;
            timeline_state.set(timeline);
        })
    };

    html! {
        <ContextProvider<Timeline> context={timeline_state.deref().clone()}>
        <div class="list_view">
            <ListSelector current_timeline={timeline_switch}/>
            <TaskList task_update={task_switch}/>
            <TaskInfo/>
        </div>
        </ContextProvider<Timeline>>
    }
}
