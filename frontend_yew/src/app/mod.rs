mod calender_component;
use calender_component::*;
use yew::prelude::*;
mod color_picker_function;
use color_picker_function::*;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct Timeline {
    pub user: String,
    pub color: (u8, u8, u8),
    pub name: String,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2>{"Calender Name | Year | Month | Day"}</h2>
            <Calender/>
            <h2>{"Select color"}</h2>
            <ColorPicker/>
        </div>
    }
}