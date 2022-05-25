// use gloo::console::log;
use cynic::{http::SurfExt, MutationBuilder, QueryBuilder};
use std::ops::Deref;
use web_sys::HtmlInputElement as InputElement;
use weblog::*;
use yew::prelude::*;
use yew::Callback;
use yew_hooks::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use super::gql::query::*;
use super::task_item::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub task_update: Callback<Task>,
}

#[function_component(TaskList)]
pub fn task_list(props: &Props) -> Html {
    let timeline_context = use_context::<UserTimeline>();
    let rf_first = use_state(|| true);
    let task_title = use_state(|| "".to_owned());

    let tasks = {
        let id = timeline_context.as_ref().unwrap().clone().timeline_id;
        let operation = GetTasksById::build(GetTasksByIdArguments { timeline_id: id });
        use_async(async move {
            let data = surf::post("http://localhost/api/graphql")
                .run_graphql(operation)
                .await
                .expect("Could not get tasks")
                .data;

            if let Some(t) = data {
                return Ok(Rc::new(RefCell::new(t.get_tasks_by_id)));
            }
            Err("Could not get tasks.")
        })
    };

    // TODO: Read from context into tasks here.

    let onkeypress = {
        let tasks = tasks.clone();
        let task_title = task_title.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let mut tasklist = tasks.data.as_ref().unwrap().clone();
                let input: InputElement = e.target_unchecked_into();
                if input.value() != "" {
                    let value = input.value();
                    input.set_value("");
                    task_title.set(value);
                } else {
                }
            } else {
            }
        })
    };

    let ondblclick = {
        Callback::from(|e: MouseEvent| {
            console_log!("doubleclicked");
        })
    };

    let task_switch = {
        let message = props.task_update.clone();
        let tasks = tasks.clone();
        Callback::from(move |taskid: i32| {
            let tasks = tasks.data.as_ref().unwrap().clone();
            for t in tasks.borrow().iter() {
                if t.id == taskid {
                    message.emit(t.clone());
                    break;
                }
            }
        })
    };

    {   
        let tasks = tasks.clone();
        use_effect(move || {
            if *rf_first {
                tasks.run();
                rf_first.set(false);
            }
            || {}
        });
    }

    html! {
        <div class="task_list">
            <h2 {ondblclick}>{timeline_context.unwrap_or(UserTimeline::default()).title}</h2>

            <input
                type="new_todo"
                placeholder="Add a new task"
                {onkeypress}
            />
            {
                if let Some(tasks) = &tasks.data {
                    html! {
                        <div class="item_list">
                        {
                            for tasks.borrow().iter().map(|task|
                                html! {
                                    <TaskItem id={task.id.to_string()} title={task.title.clone()} get_task_name={task_switch.clone()}/>
                                }
                            )
                        }
                        </div>
                    }
                } else {
                    html! {}
                }
            }

        </div>
    }
}
