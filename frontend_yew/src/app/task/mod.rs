use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub id: String,
    pub get_task_name: Callback<i32>,
}

#[function_component(Task)]
pub fn task(props: &Props) -> Html {
    let onclick = {
        let get_task_name = props.get_task_name.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlButtonElement>();
            let value = input.id();
            get_task_name.emit(value.parse().unwrap());
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
            <button id={props.id.clone()} name={props.title.clone()} onclick={onclick.clone()}>{props.title.clone()}</button>
        </div>
    }
}
