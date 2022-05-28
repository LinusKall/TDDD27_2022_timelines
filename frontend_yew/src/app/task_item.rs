use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub id: i32,
    pub done: bool,
    pub get_task_name: Callback<i32>,
    pub get_id_delete: Callback<i32>,
    pub get_task_done: Callback<(i32, bool)>,
}

#[function_component(TaskItem)]
pub fn task_item(props: &Props) -> Html {
    let onclick = {
        let get_task_name = props.get_task_name.clone();
        let id = props.id;
        Callback::from(move |_: MouseEvent| {
            get_task_name.emit(id);
        })
    };

    let delete_task = {
        let get_id_delete = props.get_id_delete.clone();
        let id = props.id;
        Callback::from(move |_: MouseEvent| {
            get_id_delete.emit(id);
        })
    };

    let task_done = {
        let get_task_done = props.get_task_done.clone();
        let id = props.id;
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            let value =  (id, input.checked());
            get_task_done.emit(value);
        }) 
    };

    html! {
        <div class="task_item" styles="display: block;">

            <label class={"checkbox"} for={format!("{}_{}", props.id, &props.title)}>
                <input
                    type="checkbox"
                    class={"checkbox_input"}
                    id={format!("{}_{}", props.id, &props.title)}
                    name={props.title.clone()}
                    checked={props.done}
                    onchange={task_done}
                />
                <div class="checkbox_box"></div>
            </label>
            <button
                class={"task_body"}
                onclick={onclick.clone()}>
                {props.title.clone()}
            </button>
            <button
                class={"task_delete"}
                onclick={delete_task.clone()}>
                {"delete"}
            </button>
        </div>
    }
}
