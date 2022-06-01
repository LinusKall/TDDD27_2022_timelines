use chrono::{offset::Local, DateTime, Utc};
use cynic::http::SurfExt;
use cynic::MutationBuilder;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use weblog::*;
use yew::prelude::*;
use yew_hooks::use_async;

use super::gql::query::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Properties {
    pub highlighted_task_update: Callback<Rc<RefCell<Task>>>,
}

#[function_component(TaskInfo)]
pub fn task_info(props: &Properties) -> Html {
    let task_context = use_context::<Rc<RefCell<Task>>>();
    let datetime = use_state(|| task_context.clone().unwrap().borrow().end_time);
    let timezone = use_state(|| *Local::now().offset());
    let switched_task = use_state(|| 0);
    let switched_timeline = use_state(|| 0);
    let updated = use_state(|| false);
    let input = use_state(|| UpdateTaskInput {
        title: None,
        body: None,
        done: None,
        end_time: None,
    });
    let body_input = use_state(|| {
        task_context
            .clone()
            .unwrap()
            .borrow()
            .body
            .as_ref()
            .unwrap_or(&"".to_owned())
            .is_empty()
    });

    let update_task = {
        let task_id = task_context.clone().unwrap().borrow().id;
        let input = input.clone();
        let updated = updated.clone();
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
                updated.set(true);
                return Ok(Rc::new(RefCell::new(t.update_task)));
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
            if k.key() == "Enter" {
                let target: HtmlInputElement = k.target_unchecked_into();
                if target.value() != "" {
                    let value = target.value();
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
            }
        })
    };

    let ondblclick = {
        let body_input = body_input.clone();
        Callback::from(move |_: MouseEvent| {
            body_input.set(true);
        })
    };

    {
        let switched_task = switched_task.clone();
        let updated = updated.clone();
        let update_task = update_task.clone();
        let datetime = datetime.clone();
        let task_context = task_context.clone();
        let body_input = body_input.clone();
        let highlighted_task_update = props.highlighted_task_update.clone();
        use_effect(move || {
            if *switched_task != task_context.clone().unwrap().borrow().id || *switched_timeline != task_context.clone().unwrap().borrow().timeline_id {
                datetime.set(task_context.clone().unwrap().borrow().end_time);
                switched_task.set(task_context.clone().unwrap().borrow().id);
                switched_timeline.set(task_context.clone().unwrap().borrow().timeline_id);
                updated.set(false);
                body_input.set(
                    task_context
                        .clone()
                        .unwrap()
                        .borrow()
                        .body
                        .as_ref()
                        .unwrap_or(&"".to_owned())
                        .is_empty(),
                )
            }
            if *updated {
                if let Some(task) = &update_task.data {
                    highlighted_task_update.emit((*task).clone());
                    updated.set(false);
                }
            }
            || {}
        })
    }

    html! {
        <div class="task-info">
            {
                if task_context.clone().unwrap().borrow().id != 0 {
                    html! {
                        <>
                            <h2>{&task_context.clone().unwrap().borrow().title}</h2>
                            {
                                if *body_input {
                                    html! {
                                        <>
                                            <p>{"Description: "}</p>
                                            <input
                                                name={"body"}
                                                placeholder="Enter a description"
                                                onkeypress={update_body}
                                            />
                                        </>
                                    }
                                } else {
                                    html! {
                                        <>
                                            <p>{"Description: "}</p>
                                            <p {ondblclick}>{task_context.clone().unwrap().borrow().body.as_ref().unwrap_or(&"".to_owned())}</p>
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
                                            <p>{"Deadline: "}</p>
                                            <input name={"endtime"} type="datetime-local" value={datetime.to_owned()} onchange={task_datetime}/>
                                        </>
                                    }
                            } else {
                                html! {
                                    <>
                                        <p>{"Deadline: "}</p>
                                        <input name={"endtime"} type="datetime-local" onchange={task_datetime}/>
                                    </>
                                }
                            }
                        }
                        {
                            if task_context.clone().unwrap().borrow().done == true {
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
