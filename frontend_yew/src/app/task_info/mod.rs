use yew::prelude::*;

use graphql_api::*;

#[function_component(TaskInfo)]
pub fn task_info() -> Html {
    let task_context = use_context::<Task>();
    html! {
        <div class="task-info">
            <h2>{task_context.unwrap_or_default().title}</h2>
        </div>
    }
}
