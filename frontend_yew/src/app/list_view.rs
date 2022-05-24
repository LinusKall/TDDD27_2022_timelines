use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use std::ops::Deref;
use yew::prelude::*;
use yew::ContextProvider;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use super::list_selector::*;
use super::task_info::*;
use super::task_list::*;
use super::Route;
use super::UserId;

mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

#[derive(cynic::FragmentArguments)]
struct GetUserTimelinesArguments {
    id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserTimelinesArguments"
)]
struct GetUserTimelinesById {
    #[arguments(id = &args.id)]
    get_usertimelinesbyid: UserTimeline,
}

#[function_component(ListView)]
pub fn list_view() -> Html {
    let user_id = use_context::<UserId>().expect("No context found.");
    let first_render = use_state(|| true);
    let timeline_id = use_state(|| 0);
    let task_id = use_state(|| 0);
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
        let id = user_id.clone();
        let operation = GetUserTimelinesById::build(GetUserTimelinesArguments { id });
        use_async(async move {
            let data = surf::post("http://localhost/api/graphql")
                .run_graphql(operation)
                .await
                .expect("Could not get User Timelines")
                .data;

            if let Some(utl) = data {
                return Ok(utl.get_usertimelinesbyid);
            }
            Err("Could not fetch user Timelines.")
        })
    };

    let task_request = {
        let id = task_id.clone();
        let operation = GetTask::build(GetTaskArguments { id });
        use_async(async move {
            let data = surf::post("http://localhost/api/graphql")
                .run_graphql(operation)
                .await
                .expect("Could not get task")
                .data
                .unwrap();

            if let task = data.get_user_data {
                return Ok(task);
            }
            Err("Could not fetch task.")
        })
    };

    use_effect(move || {
        if *first_render {
            usertimelines.run();
            first_render.set(false);
        }
        || {}
    });

    let timelines = use_state(|| Vec::new());
    let timeline_state = use_state(Timeline::default);
    let highlited_task = use_state(Task::default);

    // TODO: Change to look at timelineID
    let timeline_switch = {
        let timeline_state = timeline_state.clone();
        let usertimelines = usertimelines.clone();
        Callback::from(move |id: i32| {
            let mut timeline = timeline_state.deref().clone();
            let timelines = usertimelines.data.unwrap();
            for t in timelines.iter() {
                if t.id == id {
                    timeline.title = t.title;
                    timeline_state.set(timeline);
                    break;
                }
            }
            timeline.title = id;
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
            {if user_data.loading {
                html! {<h1>{ " Loading..." }</h1>}
            }
            else if let Some(_) = &user_data.data {
                html! {
                    <>
                        <ContextProvider<Vec<UserTimeline>> context={usertimelines.data.clone()}>
                            <ListSelector current_timeline={timeline_switch} added_timeline={timeline_add}/>
                        </ContextProvider<Vec<UserTimeline>>
                        <ContextProvider<UserTimeline> context={timeline_state.deref().clone()}>
                            <TaskList task_update={task_switch} add_task={task_add}/>
                        </ContextProvider<UserTimeline>>
                        <ContextProvider<Task> context={highlited_task.deref().clone()}>
                            <TaskInfo/>
                        </ContextProvider<Task>>
                    </>
                }
            }}
        </div>
    }
}
