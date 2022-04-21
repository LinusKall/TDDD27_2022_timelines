use yew::prelude::*;

#[function_component(Calender)]
pub fn calender() -> Html {
    html! {
        <div>
            <div>{"Header"}</div>
            <div>{"Days"}</div>
            <div>{"Cells"}</div>
        </div>
    }
}
