use std::ops::Deref;
use web_sys::HtmlInputElement;
#[allow(unused)]
use weblog::*;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub props_id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub color: String,
    pub get_current_timeline: Callback<i32>,
    pub get_id_delete: Callback<i32>,
    pub get_timeline_color: Callback<(i32, i32, String)>,
}

#[function_component(TimelineItem)]
pub fn timeline_item(props: &Props) -> Html {
    let color = use_state(|| props.color.to_owned());

    let onclick = {
        let get_current_timeline = props.get_current_timeline.clone();
        let id = props.timeline_id;
        Callback::from(move |_: MouseEvent| {
            get_current_timeline.emit(id);
        })
    };

    let delete_timeline = {
        let get_id_delete = props.get_id_delete.clone();
        let id = props.timeline_id;
        Callback::from(move |_: MouseEvent| {
            get_id_delete.emit(id);
        })
    };

    let change_color = {
        let props_id = props.props_id;
        let timeline_id = props.timeline_id;
        let color = color.clone();
        let get_timeline_color = props.get_timeline_color.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            color.set(input.value());
            get_timeline_color.emit((props_id, timeline_id, input.value()));
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
                onclick={onclick}>
                {&props.title}
            </button>
            <button
                class={"timeline_delete"}
                onclick={delete_timeline}>
                {"delete"}
            </button>
        </div>
    }
}
