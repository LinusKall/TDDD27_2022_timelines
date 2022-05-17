use super::list_selector::*;
use super::task_info::*;
use super::task_list::*;
use std::ops::Deref;
use graphql_api::*;
use yew::prelude::*;
use yew::ContextProvider;

#[function_component(ListView)]
pub fn list_view() -> Html {
    let user_data = use_state(UserData::default);
    let timeline_state = use_state(Timeline::default);
    let highlited_task = use_state(Task::default);
    // TODO: Read users data into timeline_state.

    let timeline_switch = {
        let timeline_state = timeline_state.clone();
        Callback::from(move |name: String| {
            let mut timeline = timeline_state.deref().clone();
            timeline.title = name;
            timeline_state.set(*timeline);
        })
    };
    let timeline_add = {
        let user_data = user_data.clone();
        Callback::from(move |timelinename: String| {
            let mut ud = user_data.deref().clone();
            let mut timeline = Timeline::default();
            timeline.title = timelinename;
            ud.timelines.push(timeline);
            user_data.set(*ud);
            // TODO: Set correct new id to timeline
        })
    };
    let task_switch = {
        let timeline_state = timeline_state.clone();
        Callback::from(move |task: Task| {
            let mut timeline = timeline_state.deref().clone();
            let mut task = Task::default();
            tasks.title = taskname; 
            timeline.tasks.push(task);
            ud.timelines.push(timeline);
            timeline_state.set(*timeline);
        })
    };

    html! {
        <ContextProvider<gql::Timeline> context={timeline_state.deref().clone()}>
        <div class="list_view">
            <ListSelector current_timeline={timeline_switch} added_timeline={timeline_add}/>
            <TaskList task_update={task_switch}/>
            <TaskInfo/>
        </div>
        </ContextProvider<gql::Timeline>>
    }
}


