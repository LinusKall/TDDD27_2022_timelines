pub mod task_list_component;
pub mod list_selector_component;

use std::ops::Deref;

use gloo::console::log;
use yew::prelude::*;
use task_list_component::*;
use list_selector_component::*;
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
            <ListSelectorComponent chosen_timeline={timeline_switch}/>
            <TaskListComponent/>
        </ContextProvider<Timeline>>
    }
}