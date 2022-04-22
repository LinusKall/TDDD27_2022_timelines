use yew::prelude::*;
pub mod list_selector;
pub mod list_view;
pub mod task;
pub mod task_info;
pub mod task_list;

use list_view::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ListView/>
    }
}
