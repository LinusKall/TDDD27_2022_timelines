// use gloo::console::log;
use super::task::Task;
use graphql_api as gql;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use yew::Callback;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub add_task: Callback<String>,
    pub task_update: Callback<i32>,
}

#[function_component(TaskList)]
pub fn task_list(props: &Props) -> Html {
    let tasks = use_state(|| Vec::new());
    let timeline_context = use_context::<gql::Timeline>();
    // TODO: Read from context into tasks here.

    let onkeypress = {
        let tasks = tasks.clone();
        let add_task = props.add_task.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let mut tasklist = (*tasks).clone();
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    tasklist.push(value.to_owned());
                    tasks.set(tasklist);
                    add_task.emit(value.to_owned());
                } else {
                }
            } else {
            }
        })
    };

    let task_switch = {
        let message = props.task_update.clone();
        Callback::from(move |taskid: i32| {
            message.emit(taskid);
        })
    };

    html! {
        <div class="task_list">
            <h2>{timeline_context.unwrap_or_default().title}</h2>

            <input
                type="new_todo"
                placeholder="Add a new task"
                {onkeypress}
            />

            <div class="item_list">
                {
                    for (*tasks).iter().map(|task|
                        html! {
                            <Task id={"1"} title={task.clone()} get_task_name={task_switch.clone()}/>
                        }
                    )
                }
            </div>
        </div>
    }
}
