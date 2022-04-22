use yew::prelude::*;

use super::Timeline;

#[function_component(TaskInfo)]
pub fn task_info() -> Html {
    let timeline_context = use_context::<Timeline>();
    html! {
        <div class="task-info">
            <h2>{timeline_context.unwrap_or_default().task}</h2>
        </div>
    }
}
