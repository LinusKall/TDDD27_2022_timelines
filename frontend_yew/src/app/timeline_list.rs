use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

use super::gql::query::*;
use super::timeline_item::TimelineItem;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub current_timeline: Callback<i32>,
    pub added_timeline: Callback<String>,
    pub get_id_delete: Callback<i32>,
    pub get_timeline_color: Callback<(i32, i32, String)>,
}

#[function_component(TimelineList)]
pub fn timeline_list(props: &Props) -> Html {
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

    let update_current_timeline = {
        let current_timeline = props.current_timeline.clone();
        Callback::from(move |timeline_id| {
            current_timeline.emit(timeline_id);
        })
    };

    let delete_timeline = {
        let delete_timeline = props.get_id_delete.clone();
        Callback::from(move |timeline_id| {
            delete_timeline.emit(timeline_id);
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
                            <TimelineItem
                                props_id={timeline.props_id}
                                timeline_id={timeline.timeline_id} 
                                title={timeline.title.to_owned()}
                                color={timeline.color.to_owned()}
                                get_current_timeline={update_current_timeline.clone()}
                                get_id_delete={delete_timeline.clone()}
                                get_timeline_color={props.get_timeline_color.clone()}/>
                        })
                }
            </div>
        </div>
    }
}
