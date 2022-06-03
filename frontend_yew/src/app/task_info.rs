use chrono::{offset::Local, DateTime, Utc};
use cynic::http::SurfExt;
use cynic::MutationBuilder;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use weblog::*;
use yew::prelude::*;
use yew_hooks::use_async;

use super::gql::query::*;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    pub current_task: Option<Task>,
    pub update: Callback<UpdateTaskInput>,
}

#[function_component(TaskInfo)]
pub fn task_info(props: &Props) -> Html {
    let timezone = use_state(|| *Local::now().offset());
    let body_ref = use_node_ref();
    let body_input = use_state(|| props.current_task.is_some() && props.current_task.as_ref().unwrap().body.is_none());

    let update_end_time = {
        let timezone = timezone.clone();
        let update = props.update.clone();
        let task_id = props.current_task.clone().unwrap().id;
        Callback::from(move |e: Event| {
            let value = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            let value = format!("{}:00{}", value, &*timezone);
            let newtime = DateTime::parse_from_rfc3339(&value);
            update.emit(UpdateTaskInput {
                task_id,
                title: None,
                body: None,
                done: None,
                end_time: Some(newtime.unwrap().with_timezone(&Utc)),
            });
        })
    };

    let update_body = {
        let update = props.update.clone();
        let task_id = props.current_task.clone().unwrap().id;
        let body_input = body_input.clone();
        Callback::from(move |k: KeyboardEvent| {
            let value = k.target_unchecked_into::<HtmlInputElement>().value();
            if k.shift_key() && k.key() == "Enter"{
                if value.is_empty() {
                    update.emit(UpdateTaskInput {
                        task_id,
                        title: None,
                        body: None,
                        done: None,
                        end_time: None,
                    });
                    body_input.set(true);
                } else {
                    update.emit(UpdateTaskInput {
                        task_id,
                        title: None,
                        body: Some(value),
                        done: None,
                        end_time: None,
                    });
                    body_input.set(false);
                }
            }
        })
    };

    let make_mutable = {
        let body_input = body_input.clone();
        Callback::from(move |_: MouseEvent| {
            body_input.set(true);
        })
    };

    let resize = {
        let body_ref = body_ref.clone();
        Callback::from(move |_e: InputEvent| {
            // let elem = e.target_unchecked_into::<HtmlInputElement>();
            let elem = body_ref.cast::<HtmlInputElement>().unwrap();
            elem.style().set_property("height", "0").unwrap();
            elem.style()
                .set_property("height", &format!("{}px", elem.scroll_height()))
                .unwrap();
        })
    };

    {
        let body_ref = body_ref.clone();
        use_effect(move || {
            if let Some(elem) = body_ref.cast::<HtmlInputElement>() {
                elem.style().set_property("height", "0").unwrap();
                elem.style()
                    .set_property("height", &format!("{}px", elem.scroll_height()))
                    .unwrap();
            }
            || {}
        })
    }

    html! {
        <div class="task_info">
            {
                if props.current_task.is_some() {
                    html! {
                        <>
                            <h2>{&props.current_task.as_ref().unwrap().title}</h2>
                            {
                                if *body_input {
                                    let cur_task = props.current_task.clone().unwrap();
                                    html! {
                                        <>
                                            <p><b>{"Description: "}</b></p>
                                            <textarea
                                                name={"body"}
                                                class="task_body_input"
                                                ref={body_ref.clone()}
                                                oninput={resize.clone()}
                                                value={cur_task.body.clone().unwrap_or("".to_owned())}
                                                placeholder="Describe your task! (Shift+Enter to finish)"
                                                onkeypress={update_body}
                                            />
                                        </>
                                    }
                                } else {
                                    let cur_task = props.current_task.clone().unwrap();
                                    html! {
                                        <>
                                            <p><b>{"Description (Double click to edit): "}</b></p>
                                            <p
                                                class="task_body"
                                                ondblclick={make_mutable}>
                                                {cur_task.body.clone().unwrap_or("".to_owned())}
                                            </p>
                                        </>
                                    }
                                }
                            }
                        {
                            if let Some(datetime) = props.current_task.as_ref().unwrap().end_time.clone() {
                                let datetime = datetime + *timezone;
                                let datetime = datetime.to_rfc3339();
                                let datetime = &datetime[0..datetime.len()-6];
                                html! {
                                        <>
                                            <p><b>{"Deadline: "}</b></p>
                                            <input name={"endtime"} type="datetime-local" value={datetime.to_owned()} onchange={update_end_time}/>
                                        </>
                                    }
                            } else {
                                html! {
                                    <>
                                        <p><b>{"Deadline: "}</b></p>
                                        <input name={"endtime"} type="datetime-local" onchange={update_end_time}/>
                                    </>
                                }
                            }
                        }
                        {
                            if props.current_task.as_ref().unwrap().done {
                                html! {<h3>{"Completed"}</h3>}
                            }
                            else {
                                html! {}
                            }
                        }
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
