use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use super::gql::query::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub current_timeline: Callback<i32>,
    pub added_timeline: Callback<String>,
}

#[function_component(ListSelector)]
pub fn list_selector(props: &Props) -> Html {
    let timelines_context = use_context::<Rc<RefCell<Vec<UserTimeline>>>>();
    let timelines = use_state(|| timelines_context.unwrap());
    // TODO: Read users timelines into timelines.

    let onkeypress = {
        let added_timeline = props.added_timeline.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
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
                    for timelines
                        .borrow()
                        .iter()
                        .map(|timeline| html! {
                            <button
                                onclick={onclick.clone()}
                                id={timeline.timeline_id.clone().to_string()}
                                name={timeline.title.clone()}>{timeline.title.clone()}
                            </button>
                        })
                }
            </div>
        </div>
    }
}
