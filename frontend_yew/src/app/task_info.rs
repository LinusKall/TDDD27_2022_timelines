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

#[function_component(TaskInfo)]
pub fn task_info() -> Html {
    let task_context = use_context::<Task>();
    let datetime = use_state(|| task_context.clone().unwrap_or(Task::default()).end_time);
    let timezone = use_state(|| *Local::now().offset());
    let switched_task = use_state(|| 0);
    let switched_timeline = use_state(|| 0);
    let body_ref = use_node_ref();
    let input = use_state(|| UpdateTaskInput {
        title: None,
        body: None,
        done: None,
        end_time: None,
    });
    let body_input = use_state(|| {
        task_context
            .clone()
            .unwrap_or(Task::default())
            .body
            .unwrap_or("".to_owned())
            .is_empty()
    });

    let update_task = {
        let task_id = task_context.clone().unwrap_or(Task::default()).id;
        let input = input.clone();
        use_async(async move {
            let operation = UpdateTask::build(UpdateTaskArguments {
                task_id,
                input: input.deref().clone(),
            });
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not create User Timeline")
                .data;

            if let Some(t) = data {
                return Ok(t.update_task);
            }
            Err("Could not create User Timeline.")
        })
    };

    let task_datetime = {
        let datetime = datetime.clone();
        let input = input.clone();
        let update_task = update_task.clone();
        let timezone = timezone.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            let value = format!("{}:00{}", value, &*timezone);
            let newtime = DateTime::parse_from_rfc3339(&value);
            datetime.set(Some(newtime.unwrap().with_timezone(&Utc)));
            let update = UpdateTaskInput {
                title: None,
                body: None,
                done: None,
                end_time: Some(newtime.unwrap().with_timezone(&Utc)),
            };
            input.set(update);
            update_task.run();
        })
    };

    let update_body = {
        let body_input = body_input.clone();
        let input = input.clone();
        let update_task = update_task.clone();
        Callback::from(move |k: KeyboardEvent| {
            let value = k.target_unchecked_into::<HtmlInputElement>().value();
            if k.shift_key() && k.key() == "Enter" && !value.is_empty() {
                let update = UpdateTaskInput {
                    title: None,
                    body: Some(value),
                    done: None,
                    end_time: None,
                };
                input.set(update);
                update_task.run();
                body_input.set(false);
            }
        })
    };

    let ondblclick = {
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
        let switched_task = switched_task.clone();
        let datetime = datetime.clone();
        let task_context = task_context.clone();
        let body_input = body_input.clone();
        let body_ref = body_ref.clone();
        use_effect(move || {
            let cur_task = task_context.clone().unwrap();
            if *switched_task != cur_task.id || *switched_timeline != cur_task.timeline_id {
                datetime.set(cur_task.end_time);
                switched_task.set(cur_task.id);
                switched_timeline.set(cur_task.timeline_id);
                body_input.set(
                    task_context
                        .clone()
                        .unwrap_or(Task::default())
                        .body
                        .unwrap_or("".to_owned())
                        .is_empty(),
                )
            }

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
                if task_context.clone().unwrap_or(Task::default()).id != 0 {
                    html! {
                        <>
                            <h2>{task_context.clone().unwrap_or(Task::default()).title}</h2>
                            {
                                if *body_input {
                                    let cur_task = task_context.clone().unwrap();
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
                                    let cur_task = task_context.clone().unwrap();
                                    html! {
                                        <>
                                            <p><b>{"Description (Double click to edit): "}</b></p>
                                            <p
                                                class="task_body"
                                                {ondblclick}>
                                                {cur_task.body.clone().unwrap_or("".to_owned())}
                                            </p>
                                        </>
                                    }
                                }
                            }
                        {
                            if let Some(datetime) = datetime.deref().clone() {
                                let datetime = datetime + *timezone;
                                let datetime = datetime.to_rfc3339();
                                let datetime = &datetime[0..datetime.len()-6];
                                html! {
                                        <>
                                            <p><b>{"Deadline: "}</b></p>
                                            <input name={"endtime"} type="datetime-local" value={datetime.to_owned()} onchange={task_datetime}/>
                                        </>
                                    }
                            } else {
                                html! {
                                    <>
                                        <p><b>{"Deadline: "}</b></p>
                                        <input name={"endtime"} type="datetime-local" onchange={task_datetime}/>
                                    </>
                                }
                            }
                        }
                        {
                            if task_context.clone().unwrap_or(Task::default()).done == true {
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
