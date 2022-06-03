use cynic::{http::SurfExt, MutationBuilder, QueryBuilder};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::thread::current;
use web_sys::HtmlInputElement;
use weblog::*;
use yew::prelude::*;
use yew::Callback;
use yew_hooks::prelude::*;

use super::gql::query::*;
use super::task_info::*;
use super::task_item::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub current_timeline: Option<UserTimeline>,
}

#[function_component(TaskList)]
pub fn task_list(props: &Props) -> Html {
    let current_task: UseStateHandle<Option<Task>> = use_state(|| None);

    // Async tasks
    let task_to_delete: UseStateHandle<Option<Task>> = use_state(|| None);
    let task_to_create: UseStateHandle<Option<String>> = use_state(|| None);
    let task_to_update: UseStateHandle<Option<UpdateTaskInput>> = use_state(|| None);

    // Render flags
    let rf_fetch_tasks = use_state(|| true);

    // Fetch tasks
    let tasks = {
        let timeline_id = props
            .current_timeline
            .clone()
            .unwrap_or(UserTimeline::default())
            .timeline_id;
        use_async(async move {
            let operation = GetTasksById::build(GetTasksByIdArguments { timeline_id });

            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not get tasks")
                .data;

            if let Some(t) = data {
                return Ok(t.get_tasks_by_id);
            }
            Err("Could not get tasks.")
        })
    };

    // Create new task
    let create_task_async = {
        let tasks = tasks.clone();
        let task_to_create = task_to_create.clone();
        let current_timeline = props.current_timeline.clone();
        // let rf_create_task = rf_create_task.clone();
        use_async(async move {
            let UserTimeline { timeline_id, .. } = current_timeline.unwrap();
            let title = task_to_create.deref().clone().unwrap();

            let operation = CreateTask::build(CreateTaskArguments {
                timeline_id,
                title,
                body: None,
                end_time: None,
            });

            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not create User Timeline")
                .data;

            if let Some(t) = data {
                let new = t.create_task;
                let mut new_tasks = tasks.data.as_ref().unwrap().to_vec();
                new_tasks.push(new.clone());
                tasks.update(new_tasks);
                task_to_create.set(None);
                // rf_create_task.set(true);
                return Ok(new);
            }
            Err("Could not create User Timeline.")
        })
    };

    let create_task = {
        let task_to_create = task_to_create.clone();
        let create_task_async = create_task_async.clone();
        Callback::from(move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if e.key() == "Enter" && !input.value().is_empty() {
                task_to_create.set(Some(input.value()));
                input.set_value("");
                create_task_async.run();
            }
        })
    };

    // Updated task
    let update_task_async = {
        let current_task = current_task.clone();
        let task_to_update = task_to_update.clone();
        let tasks = tasks.clone();
        use_async(async move {
            let update_input = task_to_update.deref().clone().unwrap();
            console_log!(format!("{:?}", &update_input));
            let operation = UpdateTask::build(update_input);

            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not create User Timeline")
                .data;

            console_log!(format!("{:?}", &data));

            if let Some(t) = data {
                let new = t.update_task;
                let new_tasks = tasks
                    .data
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|task| {
                        if task.id == new.id {
                            new.clone()
                        } else {
                            task.clone()
                        }
                    })
                    .collect::<Vec<Task>>();
                tasks.update(new_tasks);
                task_to_update.set(None);
                if let Some(ct) = current_timeline.deref().as_ref() {
                    if ct.id == new.id {
                        current_task.set(Some(new.clone()));
                    }
                }
                return Ok(new);
            }
            Err("Could not create User Timeline.")
        })
    };

    let update_task = {
        let task_to_update = task_to_update.clone();
        let update_task_async = update_task_async.clone();
        Callback::from(move |task_update: UpdateTaskInput| {
            task_to_update.set(Some(task_update));
            update_task_async.run();
        })
    };

    // Deletion of task
    let delete_task_async = {
        let task_to_delete = task_to_delete.clone();
        let tasks = tasks.clone();
        use_async(async move {
            let task = task_to_delete.deref().clone().unwrap();
            let task_id = task.id;
            let operation = DeleteTask::build(DeleteTaskArguments { task_id });

            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not create User Timeline")
                .data;

            if let Some(t) = data {
                let new = t.delete_task;
                let new_tasks = tasks
                    .data
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter_map(|task| {
                        if task.timeline_id == task_to_delete.as_ref().unwrap().timeline_id {
                            None
                        } else {
                            Some(task.clone())
                        }
                    })
                    .collect::<Vec<Task>>();
                tasks.update(new_tasks);
                task_to_update.set(None);
                // rf_delete_timeline.set(true);
                return Ok(new);
            }
            Err("Could not create User Timeline.")
        })
    };

    let delete_task = {
        let task_to_delete = task_to_delete.clone();
        let delete_task_async = delete_task_async.clone();
        Callback::from(move |task: Task| {
            task_to_delete.set(Some(task));
            delete_task_async.run();
        })
    };

    // Placeholder double click
    let ondblclick = {
        Callback::from(|_: MouseEvent| {
            console_log!("doubleclicked");
        })
    };

    // Switch to task
    let switch_task = {
        let current_task = current_task.clone();
        Callback::from(move |task: Task| {
            current_task.set(Some(task));
        })
    };

    // Update on every re-render
    {
        let tasks = tasks.clone();
        let rf_fetch_tasks = rf_fetch_tasks.clone();
        use_effect(move || {
            if *rf_fetch_tasks && !tasks.loading {
                tasks.run();
            }
            || {}
        });
    }

    html! {
        <div class="task_list">
            <h2 {ondblclick}>{props.current_timeline.clone().unwrap_or(UserTimeline::default()).title}</h2>

            <input
                type="new_todo"
                placeholder="Add a new task"
                onkeypress={create_task}
            />
            <div class="item_list">
            {
                if tasks.data.is_some() {
                    html! {
                        for tasks
                            .data
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|task| html! {
                                <TaskItem
                                    task={task.clone()}
                                    color={props.current_timeline.as_ref().unwrap().color.to_owned()}
                                    update={update_task.clone()}
                                    delete={delete_task.clone()}
                                    switch={switch_task.clone()}/>
                            })
                    }
                } else {
                    html! {"Loading.."}
                }
            }
            </div>

        </div>

        <TaskInfo/>
    }
}
