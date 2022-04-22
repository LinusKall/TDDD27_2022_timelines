// use gloo::console::log;
use super::task::Task;
use super::Timeline;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use yew::Callback;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub task_update: Callback<String>,
}

#[function_component(TaskList)]
pub fn task_list(props: &Props) -> Html {
    let tasks = use_state(|| Vec::new());
    let timeline_context = use_context::<Timeline>();
    // TODO: Read from context into tasks here.

    let onkeypress = {
        let tasks = tasks.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let mut tasklist = (*tasks).clone();
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    tasklist.push(value.to_owned());
                    tasks.set(tasklist);
                } else {
                }
            } else {
            }
        })
    };

    let task_switch = {
        let message = props.task_update.clone();
        Callback::from(move |name: String| {
            message.emit(name);
        })
    };

    html! {
        <div class="task_list">
            <h2>{timeline_context.unwrap_or_default().name}</h2>

            <input
                type="new_todo"
                placeholder="Add a new task"
                {onkeypress}
            />

            <div class="item_list">
                {
                    for (*tasks).iter().map(|task|
                        html! {
                            <Task title={task.clone()} get_task_name={task_switch.clone()}/>
                        }
                    )
                }
            </div>
        </div>
    }
}
