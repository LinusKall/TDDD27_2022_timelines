use yew::prelude::*;

use graphql_api as gql;

#[function_component(TaskInfo)]
pub fn task_info() -> Html {
    let timeline_context = use_context::<gql::Timeline>();
    html! {
        <div class="task-info">
            <h2>{timeline_context.unwrap_or_default().task}</h2>
        </div>
    }
}
