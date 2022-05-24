use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub current_timeline: Callback<String>,
    pub added_timeline: Callback<String>,
}

#[function_component(ListSelector)]
pub fn list_selector(props: &Props) -> Html {
    let timelines_context = use_context::<Vec<UserTimeline>>();
    let timelines = use_state(|| Vec::new());
    for t in timelines_context.unwrap_or_default().iter() {
        let temp = timelines.deref().clone();
        temp.push((t.title.clone(), t.id.clone()));
        timelines.set(temp);
    }
    // TODO: Read users timelines into timelines.

    let onkeypress = {
        let timelines = timelines.clone();
        let added_timeline = props.added_timeline.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let mut timeline_list = timelines.deref().clone();
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    timeline_list.push(value.clone());
                    timelines.set(timeline_list);
                    added_timeline.emit(value);
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
            let value = input.id();
            current_timeline.emit(value.trim().parse::<i32>().unwrap());
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
                            <button
                                onclick={onclick.clone()}
                                id={(*timeline.1).clone()}
                                name={(*timeline.0).clone()}>{timeline}
                            </button>
                        })
                }
            </div>
        </div>
    }
}
