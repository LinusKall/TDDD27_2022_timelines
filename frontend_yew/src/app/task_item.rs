use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[allow(unused)]
use weblog::*;

use super::gql::query::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub task: Task,
    pub color: String,
    pub update: Callback<UpdateTaskInput>,
    pub delete: Callback<Task>,
    pub switch: Callback<Task>,
}

#[function_component(TaskItem)]
pub fn task_item(props: &Props) -> Html {
    let select = {
        let task = props.task.clone();
        let switch = props.switch.clone();
        Callback::from(move |_: MouseEvent| {
            switch.emit(task.clone());
        })
    };

    let delete_task = {
        let task = props.task.clone();
        let delete = props.delete.clone();
        Callback::from(move |_: MouseEvent| {
            delete.emit(task.clone());
        })
    };

    let task_done = {
        let task = props.task.clone();
        let update = props.update.clone();
        let task_id = props.clone().task.id;
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            update.emit(UpdateTaskInput {
                task_id,
                title: None,
                body: None,
                done: Some(input.checked()),
                end_time: None,
            });
        })
    };

    html! {
        <div class="task_item">
            <label class={"checkbox"} for={format!("{}_{}", props.task.id, &props.task.title)}>
                <input
                    type="checkbox"
                    class={"checkbox_input"}
                    id={format!("{}_{}", props.task.id, &props.task.title)}
                    name={props.task.title.to_owned()}
                    checked={props.task.done}
                    onchange={task_done.clone()}/>
                <div
                    class="checkbox_box"
                    style={format!("--checked-color: {};", &props.color)}>
                </div>
            </label>
            <button
                class={"task_body"}
                onclick={select.clone()}>
                {&props.task.title}
            </button>
            <button
                class={"task_delete"}
                onclick={delete_task.clone()}>
                {"delete"}
            </button>
        </div>
    }
}
