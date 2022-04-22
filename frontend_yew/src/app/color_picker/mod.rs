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
            let value = input.value();
            
            let red   = u8::from_str_radix(&value[1..3], 16).unwrap();
            let green = u8::from_str_radix(&value[3..5], 16).unwrap();
            let blue  = u8::from_str_radix(&value[5..7], 16).unwrap();
            color.set((red, green, blue));
        })
    };

    html! {
        <>
            <h2 style={ format!("color: rgb({},{},{});", (*color).0, (*color).1, (*color).2) }>{&*indicator}</h2>
            <input type="color" { onchange }/>
        </>
    }
}