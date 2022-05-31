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
    console_log!(format!(
        "{}",
        task_context.clone().unwrap_or(Task::default()).created_at
    ));
    console_log!(format!(
        "{:?}",
        task_context.clone().unwrap_or(Task::default()).created_at
    ));
    console_log!(format!("{:?}", Utc::now()));

    let update_task = {
        let task_id = task_context.clone().unwrap_or(Task::default()).id;
        let input = datetime.clone();
        console_log!(format!("{:?}", *input));
        use_async(async move {
            let operation = UpdateTask::build(UpdateTaskArguments {
                task_id,
                input: UpdateTaskInput {
                    title: None,
                    body: None,
                    done: None,
                    end_time: input.deref().clone(),
                },
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
        let input = datetime.clone();
        let update_task = update_task.clone();
        let timezone = timezone.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            let value = format!("{}:00{}", value, &*timezone);
            console_log!(format!("{:?}", &value));
            let datetime = DateTime::parse_from_rfc3339(&value);
            input.set(Some(datetime.unwrap().with_timezone(&Utc)));
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
            <h2 {ondblclick}>{task_context.clone().unwrap_or(Task::default()).title}</h2>
            <h2>{task_context.unwrap_or(Task::default()).body.unwrap_or("".to_owned())}</h2>
            {
                if let Some(datetime) = datetime.deref().clone() {
                    let datetime = datetime.to_rfc3339();
                    let datetime = &datetime[0..datetime.len()-6];
                    html! {<input type="datetime-local" value={datetime.to_owned()} onchange={task_datetime}/>}
                }
                else {
                    html! {<input type="datetime-local" onchange={task_datetime}/>}
                }
            }
        </div>
    }
}
