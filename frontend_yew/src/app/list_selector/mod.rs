use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use web_sys::HtmlButtonElement;
// use super::Timeline;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub current_timeline: Callback<String>,
}

#[function_component(ListSelector)]
pub fn list_selector(props: &Props) -> Html {
    let timelines = use_state(|| Vec::new());
    // let timeline_context = use_context::<Timeline>();
    // TODO: Read users timelines into timelines.
    
    let onkeypress = {
        let timelines = timelines.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter"{
                let mut timeline_list = (*timelines).clone();
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    timeline_list.push(value);
                    timelines.set(timeline_list);
                } else { 
                }
            } else {
            }
        })
    };

    let onclick = {
        let current_timeline = props.current_timeline.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlButtonElement>();
            let value = input.name();
            current_timeline.emit(value);
        })
    };

    html! {
        <div class="list_selector">
            <input
                type="new_list"
                placeholder="Add a new timeline"
                {onkeypress}
            />

            <div class="available_timelines">
                {
                    for (*timelines)
                        .clone()
                        .iter()
                        .map(|timeline| html! {
                            <button onclick={onclick.clone()} name={(*timeline).clone()}>{timeline}</button>
                        })
                }
            </div>
        </div>
    }
}