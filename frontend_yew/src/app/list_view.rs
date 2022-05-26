use cynic::{http::SurfExt, MutationBuilder, QueryBuilder};
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;
use yew::ContextProvider;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use super::gql::query::*;
use super::list_selector::*;
use super::task_info::*;
use super::task_list::*;
use super::Route;
use super::UserId;

#[function_component(ListView)]
pub fn list_view() -> Html {
    let timeline_state = use_state(UserTimeline::default);
    let highlited_task = use_state(Task::default);
    let user_id = use_context::<UserId>().expect("No context found.");
    let rf_first = use_state(|| true);
    let timeline_title = use_state(|| "".to_owned());
    let rf_new_timeline = use_state(|| false);
    // LocalStorage::delete("timelines_user_id");

    *user_id.borrow_mut() = match LocalStorage::get("timelines_user_id") {
        Ok(uid) => uid,
        _ => {
            return html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
    };

    let usertimelines = {
        let user_id = user_id.clone();
        let operation = GetUserTimelinesById::build(GetUserTimelinesArguments {
            user_id: user_id.borrow_mut().deref().unwrap(),
        });
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not get User Timelines")
                .data;

            if let Some(utl) = data {
                return Ok(Rc::new(RefCell::new(utl.get_user_timelines_by_id)));
            }
            Err("Could not fetch user Timelines.")
        })
    };

    let new_timeline = {
        let user_id = user_id.clone();
        let timeline_title = timeline_title.deref().clone();
        let operation = CreateUserTimeline::build(CreateUserTimelineArguments {
            user_id: user_id.borrow_mut().deref().unwrap(),
            title: timeline_title,
            public: false,
        });
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not create User Timeline")
                .data;

            if let Some(tl) = data {
                return Ok(tl.create_user_timeline);
            }
            Err("Could not create User Timeline.")
        })
    };

    // TODO: Change to look at timelineID
    let timeline_switch = {
        let timeline_state = timeline_state.clone();
        let usertimelines = usertimelines.clone();
        Callback::from(move |id: i32| {
            let mut timeline = timeline_state.deref().clone();
            let timelines = usertimelines.data.as_ref().unwrap();
            for t in timelines.borrow().iter() {
                if t.timeline_id == id {
                    timeline.title = t.title.to_owned();
                    timeline.timeline_id = id;
                    timeline_state.set(timeline);
                    break;
                }
            }
        })
    };

    let timeline_add = {
        let timeline_title = timeline_title.clone();
        let new_timeline = new_timeline.clone();
        let rf_new_timeline = rf_new_timeline.clone();
        Callback::from(move |timelinename: String| {
            let new_timeline = new_timeline.clone();
            let title = timelinename;
            timeline_title.set(title);
            new_timeline.run();
            rf_new_timeline.set(true);
        })
    };

    let task_switch = {
        let highlited_task = highlited_task.clone();
        Callback::from(move |task: Task| {
            highlited_task.set(task);
        })
    };
    {
        let usertimelines = usertimelines.clone();
        use_effect(move || {
            if *rf_first {
                usertimelines.run();
                rf_first.set(false);
            }
            if *rf_new_timeline {
                if let Some(new_user_timeline) = new_timeline.data.clone() {
                    let utl = usertimelines.data.as_ref().unwrap().clone();
                    utl.borrow_mut().push(new_user_timeline);
                    rf_new_timeline.set(false);
                }
            }
            || {}
        });
    }
    html! {
        {
            if let Some(usertimelines) = usertimelines.data.as_ref() {
                html! {
                    <div class="list_view">
                        <ContextProvider<Rc<RefCell<Vec<UserTimeline>>>> context={usertimelines.clone()}>
                            <ListSelector current_timeline={timeline_switch} added_timeline={timeline_add}/>
                        </ContextProvider<Rc<RefCell<Vec<UserTimeline>>>>>

                        <ContextProvider<UserTimeline> context={timeline_state.deref().clone()}>
                            <TaskList task_update={task_switch}/>
                        </ContextProvider<UserTimeline>>

                        <ContextProvider<Task> context={highlited_task.deref().clone()}>
                            <TaskInfo/>
                        </ContextProvider<Task>>
                    </div>
                }
            } else {
                html! {<h1>{ " Loading..." }</h1>}
            }
        }
    }
}
