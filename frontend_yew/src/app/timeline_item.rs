use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[allow(unused)]
use weblog::*;

use super::gql::mutation::*;
use super::gql::query::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub user_timeline: UserTimeline,
    pub update: Callback<UpdateUserTimelineInput>,
    pub delete: Callback<UserTimeline>,
    pub switch: Callback<UserTimeline>,
}

#[function_component(TimelineItem)]
pub fn timeline_item(props: &Props) -> Html {
    let color = use_state(|| props.user_timeline.color.to_owned());

    let select = {
        let timeline = props.user_timeline.clone();
        let switch = props.switch.clone();
        Callback::from(move |_: MouseEvent| {
            switch.emit(timeline.clone());
        })
    };

    let delete_timeline = {
        let timeline = props.user_timeline.clone();
        let delete = props.delete.clone();
        Callback::from(move |_: MouseEvent| {
            delete.emit(timeline.clone());
        })
    };

    let change_color = {
        let timeline = props.user_timeline.clone();
        let update = props.update.clone();
        let color = color.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            color.set(input.value());
            update.emit(UpdateUserTimelineInput {
                props_id: timeline.props_id,
                timeline_id: timeline.timeline_id,
                title: None,
                color: Some(input.value()),
                relation: None,
            });
        })
    };

    html! {
        <div class="timeline_item">
            <input
                class={"timeline_color"}
                value={color.deref().to_owned()}
                onchange={change_color}
                type={"color"}/>
            <button
                class={"timeline_body"}
                onclick={select}>
                {&props.user_timeline.title}
            </button>
            <button
                class={"timeline_delete"}
                onclick={delete_timeline}>
                {"delete"}
            </button>
        </div>
    }
}
