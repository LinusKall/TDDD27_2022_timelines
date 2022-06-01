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

#[allow(unused)]
use weblog::*;

use super::gql::query::*;
use super::gql::mutation::*;
use super::timeline_list::*;
use super::task_info::*;
use super::task_list::*;
use super::Route;
use super::UserId;

#[function_component(ListView)]
pub fn list_view() -> Html {
    let timeline_state = use_state(UserTimeline::default);
    let highlighted_task = use_state(|| Rc::new(RefCell::new(Task::default())));
    let user_id = use_context::<UserId>().expect("No context found.");
    let rf_first = use_state(|| true);
    let timeline_title = use_state_eq(|| "".to_owned());
    let rf_new_timeline = use_state(|| false);
    let props_id = use_state(|| -1);
    let timeline_id = use_state(|| -1);
    let update_timeline_color = use_state(|| None);
    let update_timeline_title = use_state(|| None);
    let update_timeline_relation = use_state(|| None);

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
        let rf_new_timeline = rf_new_timeline.clone();
        let user_id = user_id.clone();
        let timeline_title = timeline_title.clone();
        let operation = CreateUserTimeline::build(CreateUserTimelineArguments {
            user_id: user_id.borrow_mut().deref().unwrap(),
            title: timeline_title.deref().to_owned(),
            public: false,
        });
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not create User Timeline")
                .data;

            if let Some(tl) = data {
                rf_new_timeline.set(true);
                return Ok(Some(tl.create_user_timeline));
            }
            Err("Could not create User Timeline.")
        })
    };

    let remove_timeline = {
        let rf_first = rf_first.clone();
        let props_id = props_id.deref().clone();
        let operation = DeleteUserTimeline::build(DeleteUserTimelineArguments { props_id });
        use_async(async move {
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not delete User Timeline")
                .data;

            if let Some(tl) = data {
                rf_first.set(true);
                return Ok(tl.delete_user_timeline);
            }
            Err("Could not delete User Timeline.")
        })
    };

    let timeline_switch = {
        let timeline_state = timeline_state.clone();
        let usertimelines = usertimelines.clone();
        let highlighted_task = highlighted_task.clone();
        Callback::from(move |id: i32| {
            let timelines = usertimelines.data.as_ref().unwrap();
            for t in timelines.borrow().iter() {
                if t.timeline_id == id {
                    timeline_state.set(t.clone());
                    highlighted_task.set(Rc::new(RefCell::new(Task::default())));
                    break;
                }
            }
        })
    };

    let timeline_add = {
        let timeline_title = timeline_title.clone();
        let new_timeline = new_timeline.clone();
        Callback::from(move |timelinename: String| {
            timeline_title.set(timelinename);
            new_timeline.run();
        })
    };

    let delete_timeline = {
        let timeline_state = timeline_state.clone();
        let props_id = props_id.clone();
        let remove_timeline = remove_timeline.clone();
        Callback::from(move |id: i32| {
            props_id.set(id);
            remove_timeline.run();
            timeline_state.set(UserTimeline::default());
        })
    };

    let update_timeline = {
        let props_id = props_id.clone();
        let timeline_id = timeline_id.clone();
        let update_timeline_title = update_timeline_title.clone();
        let update_timeline_color = update_timeline_color.clone();
        let update_timeline_relation = update_timeline_relation.clone();
        let rf_first = rf_first.clone();
        use_async(async move {
            let operation = UpdateUserTimeline::build(UpdateUserTimelineInput {
                props_id: *props_id,
                timeline_id: *timeline_id,
                title: update_timeline_title.deref().clone(),
                color: update_timeline_color.deref().clone(),
                relation: update_timeline_relation.deref().clone(),
            });

            let data: Option<UpdateUserTimeline> = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not update User Timeline")
                .data;

            if let Some(tl) = data {
                rf_first.set(true);
                return Ok(tl.update_user_timeline);
            }
            Err("Could not delete User Timeline.")
        })
    };

    let change_timeline_color = {
        //let timelines = usertimelines.data.clone();
        let props_id = props_id.clone();
        let timeline_id = timeline_id.clone();
        let update_timeline = update_timeline.clone();
        let update_timeline_color = update_timeline_color.clone();
        Callback::from(move |(p_id, t_id, c): (i32, i32, String)| {
            props_id.set(p_id);
            timeline_id.set(t_id);
            update_timeline_color.set(Some(c));
            update_timeline.run();

        })
    };

    let task_switch = {
        let highlighted_task = highlighted_task.clone();
        Callback::from(move |task: Rc<RefCell<Task>>| {
            highlighted_task.set(task);
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
                if let Some(Some(new_user_timeline)) = new_timeline.data.clone() {
                    let utl = usertimelines.data.as_ref().unwrap().clone();
                    utl.borrow_mut().push(new_user_timeline);
                    new_timeline.update(None);
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
                            <TimelineList
                                current_timeline={timeline_switch}
                                added_timeline={timeline_add}
                                get_id_delete={delete_timeline}
                                get_timeline_color={change_timeline_color}/>
                        </ContextProvider<Rc<RefCell<Vec<UserTimeline>>>>>

                        <ContextProvider<UserTimeline> context={timeline_state.deref().clone()}>
                            <TaskList task_update={task_switch.clone()}/>
                        </ContextProvider<UserTimeline>>

                        <ContextProvider<Rc<RefCell<Task>>> context={highlighted_task.deref().clone()}>
                            <TaskInfo highlighted_task_update={task_switch.clone()}/>
                        </ContextProvider<Rc<RefCell<Task>>>>
                    </div>
                }
            } else {
                html! {<h1>{ " Loading..." }</h1>}
            }
        }
    }
}
