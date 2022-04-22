pub mod task_list;
pub mod list_selector;

use std::ops::Deref;

// use gloo::console::log;
use yew::prelude::*;
use task_list::*;
use list_selector::*;
use yew::ContextProvider;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Timeline {
    pub user: String,
    pub color: (u8, u8, u8),
    pub name: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let timeline_state = use_state(Timeline::default);
    // TODO: Read users data into timeline_state.

    let timeline_switch = {
        let timeline_state = timeline_state.clone();
        Callback::from(move |name: String| {
            let mut timeline = timeline_state.deref().clone();
            timeline.name = name;
            timeline_state.set(timeline);
        })
    };

    html! {
        <ContextProvider<Timeline> context={timeline_state.deref().clone()}>
        <div class="list_view">
            <ListSelector current_timeline={timeline_switch}/>
            <TaskList/>
        </div>
        </ContextProvider<Timeline>>
    }
}