use yew::prelude::*;
use web_sys::HtmlInputElement;

#[function_component(ColorPicker)]
pub fn color_picker() -> Html {
    let indicator = use_state_eq(|| "O".to_owned());
    let color = use_state_eq(|| (0_u8, 0_u8, 0_u8));
    let onchange = {
        //let data = data.clone();
        let color = color.clone();
        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let name = input.name();
            let value = input.value();

            if let Some(color_val) = value.parse().ok() {
                match name.as_str() {
                    "red"   => color.set((color_val, (*color).1, (*color).2)),
                    "green" => color.set(((*color).0, color_val, (*color).2)),
                    "blue"  => color.set(((*color).0, (*color).1, color_val)),
                    _       => {}
                }
            }
        })
    };

    html! {
        <>
            <h2 style={ format!("color: rgb({},{},{});", (*color).0, (*color).1, (*color).2) }>{&*indicator}</h2>
            <input name="red" placeholder="Red" onchange={onchange.clone()}/>
            <input name="green" placeholder="Green" onchange={onchange.clone()}/>
            <input name="blue" placeholder="Blue" onchange={onchange.clone()}/>
        </>
    }
}