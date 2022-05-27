use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

use super::gql::query::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub current_timeline: Callback<i32>,
    pub added_timeline: Callback<String>,
    pub get_id_delete: Callback<i32>,
}

#[function_component(ListSelector)]
pub fn list_selector(props: &Props) -> Html {
    let timelines_context = use_context::<Rc<RefCell<Vec<UserTimeline>>>>();

    let onkeypress = {
        let added_timeline = props.added_timeline.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    added_timeline.emit(value);
                }
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

    let delete_timeline = {
        let get_id_delete = props.get_id_delete.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlButtonElement>();
            let value = input.id();
            get_id_delete.emit(value.trim().parse::<i32>().unwrap());
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
                    for timelines_context
                        .unwrap()
                        .borrow()
                        .iter()
                        .map(|timeline| html! {
                            <>
                                <button
                                    class={"timelinebody"}
                                    onclick={onclick.clone()}
                                    id={timeline.timeline_id.clone().to_string()}
                                </button>
                                <button
                                    class={"timelinedelete"}
                                    onclick={delete_timeline.clone()}
                                    id={timeline.props_id.clone().to_string()}
                                >{"delete"}
                                </button>
                            </>
                        })
                }
            </div>
        </div>
    }
}
