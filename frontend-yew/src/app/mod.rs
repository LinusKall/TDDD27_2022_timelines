pub mod task_list_component;
pub mod list_selector_component;

use yew::prelude::*;
use task_list_component::*;
use list_selector_component::*;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <ListSelector/>
            <TaskListComponent/>
        </div>
    }
}