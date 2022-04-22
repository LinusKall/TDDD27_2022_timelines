use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    title: String,
}

#[function_component(Task)]
pub fn task(props: &Props) -> Html {
    html! {
        <div class="task">
            <input type="checkbox" class={"checkbox"} id={props.title} name={props.title}/>
            <label for={props.title}></label>
            <button>{props.title}</button>
        </div>
    }
}
