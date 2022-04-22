// use gloo::console::log;
use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use yew::Callback;

use super::Timeline;

#[function_component(TaskList)]
pub fn task_list() -> Html {
    let tasks = use_state(|| Vec::new());
    let timeline_context = use_context::<Timeline>();
    let tasks_clone = tasks.clone();
    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter"{
            let tasks = tasks_clone.clone();
            let mut list = (*tasks).clone();
            let input: InputElement = e.target_unchecked_into();
            if input.value() != "" {
                let value = input.value();
                input.set_value("");
                list.push(value.to_owned());
                tasks.set(list);
            } else { 
            }
        } else {
        }
    });
    html! {
        <div class="task_list">
            <h2>{timeline_context.unwrap_or_default().name}</h2> 
            <input
                type="new_todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
            <div class="item_list">
                { 
                    for (*tasks).iter().map(|task| 
                        html! {
                            <div styles="display: block;">
                                <input type="checkbox" id={task.clone()} name={task.clone()}/>
                                <label for={task.clone()}>{task}</label>
                            </div>
                        }
                    )
                }
            </div>
        </div>
    }
}