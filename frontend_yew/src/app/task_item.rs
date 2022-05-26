use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub id: String,
    pub get_task_name: Callback<i32>,
    pub get_id_delete: Callback<i32>,
}

#[function_component(TaskItem)]
pub fn task_item(props: &Props) -> Html {
    let onclick = {
        let get_task_name = props.get_task_name.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlButtonElement>();
            let value = input.id();
            get_task_name.emit(value.trim().parse::<i32>().unwrap());
        })
    };

    let delete_task = {
        let get_id_delete = props.get_id_delete.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlButtonElement>();
            let value = input.id();
            get_id_delete.emit(value.trim().parse::<i32>().unwrap());
        })
    };

    html! {
        <div class="task" styles="display: block;">
            <input
                type="checkbox"
                class={"checkbox"}
                id={props.title.clone()}
                name={props.title.clone()}
            />
            <label for={props.title.clone()}></label>
            <button class={"taskbody"} id={props.id.clone()} name={props.title.clone()} onclick={onclick.clone()}>{props.title.clone()}</button>
            <button class={"taskdelete"} id={props.id.clone()} name={props.title.clone()} onclick={delete_task.clone()}>{"delete"}</button>
        </div>
    }
}
