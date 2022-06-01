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
    let switched = use_state(|| 0);
    let input = use_state(|| UpdateTaskInput {
        title: None,
        body: None,
        done: None,
        end_time: None,
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

    {
        let switched = switched.clone();
        let datetime = datetime.clone();
        let task_context = task_context.clone();
        use_effect(move || {
            if *switched != task_context.clone().unwrap_or(Task::default()).id {
                datetime.set(task_context.clone().unwrap_or(Task::default()).end_time);
                switched.set(task_context.clone().unwrap_or(Task::default()).id);
            }
            || {}
        })
    }

    let ondblclick = { Callback::from(|_: MouseEvent| {}) };
    html! {
        <div class="task-info">
            {
                if task_context.clone().unwrap_or(Task::default()).id != 0 {
                    html! {
                        <>
                            <h2 {ondblclick}>{task_context.clone().unwrap_or(Task::default()).title}</h2>
                            <p>{"Description: "}</p>
                            <input name={"body"} value={task_context.clone().unwrap_or(Task::default()).body.unwrap_or("".to_owned())}/>
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
