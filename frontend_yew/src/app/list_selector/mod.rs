use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use web_sys::HtmlButtonElement;
// use super::Timeline;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub chosen_timeline: Callback<String>,
}

#[function_component(ListSelector)]
pub fn list_selector(props: &Props) -> Html {
    let timelines = use_state(|| vec!["List1".to_owned(), "List2".to_owned(), "List3".to_owned()]);
    let timelines_clone = timelines.clone();
    // let timeline_context = use_context::<Timeline>();
    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter"{
            let timelines = timelines_clone.clone();
            let mut list = (*timelines).clone();
            let input: InputElement = e.target_unchecked_into();
            if input.value() != "" {
                let value = input.value();
                input.set_value("");
                list.push(value);
                timelines.set(list);
            } else { 
            }
        } else {
        }
    });
    let chosen_timeline_clone = props.chosen_timeline.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        let target = e.target().unwrap();
        let input = target.unchecked_into::<HtmlButtonElement>();
        let value = input.name();
        chosen_timeline_clone.emit(value);
    });
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