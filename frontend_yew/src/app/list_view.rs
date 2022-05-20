use std::ops::Deref;
use graphql_api::*;
use yew::prelude::*;
use yew::ContextProvider;
use gloo_storage::Storage;
use gloo::storage::LocalStorage;
use yew_router::prelude::*;


use super::list_selector::*;
use super::task_info::*;
use super::task_list::*;
use super::Route;
use super::UserId;


#[function_component(ListView)]
pub fn list_view() -> Html {
    let user_data = use_state(UserData::default);
    let timeline_state = use_state(Timeline::default);
    let highlited_task = use_state(Task::default);
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
    // TODO: Read users data into timeline_state.

    let timeline_switch = {
        let timeline_state = timeline_state.clone();
        Callback::from(move |name: String| {
            let mut timeline = timeline_state.deref().clone();
            timeline.title = name;
            timeline_state.set(timeline);
        })
    };
    let timeline_add = {
        let user_data = user_data.clone();
        Callback::from(move |timelinename: String| {
            let mut ud = user_data.deref().clone();
            let mut timeline = Timeline::default();
            timeline.title = timelinename;
            ud.timelines.push(timeline);
            user_data.set(ud);
            // TODO: Set correct new id to timeline
        })
    };
    let task_add = {
        let timeline_state = timeline_state.clone();
        Callback::from(move |taskname: String| {
            let mut timeline = timeline_state.deref().clone();
            let mut task = Task::default();
            task.title = taskname;
            task.id = 1;
            timeline.tasks.push(task);
            timeline_state.set(timeline);
            // TODO: Set correct new id to task
        })
    };
    let task_switch = {
        let highlited_task = highlited_task.clone();
        let timeline_state = timeline_state.clone();
        Callback::from(move |taskid: i32| {
            let mut task = highlited_task.deref().clone();
            let timeline = timeline_state.deref().clone();
            for t in timeline.tasks.iter() {
                if t.id == taskid {
                    task.title = t.title.clone();
                    highlited_task.set(task);
                    break;
                }
            }
        })
    };

    html! {
        <div class="list_view">
            <ContextProvider<Timeline> context={timeline_state.deref().clone()}>
                <ListSelector current_timeline={timeline_switch} added_timeline={timeline_add}/>
                <TaskList task_update={task_switch} add_task={task_add}/>
            </ContextProvider<Timeline>>
            <ContextProvider<Task> context={highlited_task.deref().clone()}>
                <TaskInfo/>
            </ContextProvider<Task>>
        </div>
    }
}


