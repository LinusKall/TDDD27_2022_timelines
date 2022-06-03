use cynic::http::SurfExt;
use cynic::MutationBuilder;
use cynic::QueryBuilder;
use std::ops::Deref;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use yew_hooks::use_async;

#[allow(unused)]
use weblog::*;

use super::gql::mutation::*;
use super::gql::query::*;
use super::task_list::*;
use super::timeline_item::TimelineItem;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub user_id: i32,
}

#[function_component(TimelineList)]
pub fn timeline_list(props: &Props) -> Html {
    let current_timeline: UseStateHandle<Option<UserTimeline>> = use_state(|| None);

    // Async tasks
    let timeline_to_delete: UseStateHandle<Option<UserTimeline>> = use_state(|| None);
    let timeline_to_create: UseStateHandle<Option<String>> = use_state(|| None);
    let timeline_to_update: UseStateHandle<Option<UpdateUserTimelineInput>> = use_state(|| None);

    // Render flags
    let rf_fetch_timelines = use_state(|| true);
    // let rf_create_timeline = use_state(|| false);
    // let rf_delete_timeline = use_state(|| false);
    // let rf_update_timeline = use_state(|| false);

    // Fetch Users timelines
    let user_timelines = {
        let rf_fetch_timelines = rf_fetch_timelines.clone();
        let user_id = props.user_id;
        use_async(async move {
            let operation = GetUserTimelinesById::build(GetUserTimelinesArguments { user_id });

            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not get User Timelines")
                .data;

            if let Some(utl) = data {
                rf_fetch_timelines.set(false);
                return Ok(utl.get_user_timelines_by_id);
            }
            Err("Could not fetch user Timelines.")
        })
    };

    // Create new timeline
    let create_timeline_async = {
        let user_id = props.user_id;
        let timeline_to_create = timeline_to_create.clone();
        // let rf_create_timeline = rf_create_timeline.clone();
        let user_timelines = user_timelines.clone();
        use_async(async move {
            let title = timeline_to_create.deref().clone().unwrap();
            let operation = CreateUserTimeline::build(CreateUserTimelineArguments {
                user_id,
                title,
                public: false,
            });

            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not create User Timeline")
                .data;

            if let Some(tl) = data {
                let new = tl.create_user_timeline;
                let mut new_timelines = user_timelines.data.as_ref().unwrap().to_vec();
                new_timelines.push(new.clone());
                user_timelines.update(new_timelines);
                timeline_to_create.set(None);
                // rf_create_timeline.set(true);
                return Ok(new);
            }
            Err("Could not create User Timeline.")
        })
    };

    let create_timeline = {
        let to_create_timeline = timeline_to_create.clone();
        let create_timeline_async = create_timeline_async.clone();
        Callback::from(move |e: KeyboardEvent| {
            let input: InputElement = e.target_unchecked_into();
            if e.key() == "Enter" && !input.value().is_empty() {
                to_create_timeline.set(Some(input.value()));
                input.set_value("");
                create_timeline_async.run();
            }
        })
    };

    // Update current timeline
    let update_timeline_async = {
        let timeline_to_update = timeline_to_update.clone();
        let user_timelines = user_timelines.clone();
        use_async(async move {
            let update_input = timeline_to_update.deref().clone().unwrap();
            let operation = UpdateUserTimeline::build(update_input);

            let data: Option<UpdateUserTimeline> =
                surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                    .run_graphql(operation)
                    .await
                    .expect("Could not update User Timeline")
                    .data;

            if let Some(tl) = data {
                let new = tl.update_user_timeline;
                let new_timelines = user_timelines
                    .data
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|t| {
                        if t.timeline_id == new.timeline_id {
                            new.clone()
                        } else {
                            t.clone()
                        }
                    })
                    .collect::<Vec<UserTimeline>>();
                user_timelines.update(new_timelines);
                timeline_to_update.set(None);
                return Ok(new);
            }
            Err("Could not delete User Timeline.")
        })
    };

    let update_timeline = {
        let timeline_to_update = timeline_to_update.clone();
        let update_timeline_async = update_timeline_async.clone();
        Callback::from(move |timeline_update: UpdateUserTimelineInput| {
            timeline_to_update.set(Some(timeline_update));
            update_timeline_async.run();
        })
    };

    // Deletion of timeline
    let delete_timeline_async = {
        let timeline_to_delete = timeline_to_delete.clone();
        // let rf_delete_timeline = rf_delete_timeline.clone();
        let user_timelines = user_timelines.clone();
        use_async(async move {
            let timeline = timeline_to_delete.deref().clone().unwrap();
            let props_id = timeline.props_id;
            let operation = DeleteUserTimeline::build(DeleteUserTimelineArguments { props_id });
            let data = surf::post(format!("{}/api/graphql", crate::app::LOCALHOST))
                .run_graphql(operation)
                .await
                .expect("Could not delete User Timeline")
                .data;

            if let Some(tl) = data {
                let new = tl.delete_user_timeline;
                let new_timelines = user_timelines
                    .data
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter_map(|t| {
                        if t.timeline_id == timeline_to_delete.as_ref().unwrap().timeline_id {
                            None
                        } else {
                            Some(t.clone())
                        }
                    })
                    .collect::<Vec<UserTimeline>>();
                user_timelines.update(new_timelines);
                timeline_to_update.set(None);
                // rf_delete_timeline.set(true);
                return Ok(new);
            }
            Err("Could not delete User Timeline.")
        })
    };

    let delete_timeline = {
        let timeline_to_delete = timeline_to_delete.clone();
        let delete_timeline_async = delete_timeline_async.clone();
        Callback::from(move |timeline: UserTimeline| {
            timeline_to_delete.set(Some(timeline));
            delete_timeline_async.run();
        })
    };

    // Switch highlighted timeline
    let switch_timeline = {
        let current_timeline = current_timeline.clone();
        Callback::from(move |timeline: UserTimeline| {
            current_timeline.set(Some(timeline));
        })
    };

    // Update on every re-render
    {
        let user_timelines = user_timelines.clone();
        let rf_fetch_timelines = rf_fetch_timelines.clone();
        // let rf_create_timeline = rf_create_timeline.clone();
        // let rf_delete_timeline = rf_delete_timeline.clone();
        // let rf_update_timeline = rf_update_timeline.clone();
        use_effect(move || {
            if *rf_fetch_timelines && !user_timelines.loading {
                user_timelines.run();
            }
            // if *rf_create_timeline {
            //     rf_create_timeline.set(false);
            // }
            // if *rf_delete_timeline {
            //     rf_delete_timeline.set(false);
            // }
            // if *rf_update_timeline {
            //     rf_update_timeline.set(false);
            // }
            || {}
        });
    }

    // Render component
    html! {
        <>
            <div class="timeline_list">

                <input
                    type="new_list"
                    placeholder="Add a new timeline"
                    onkeypress={create_timeline}
                />

                <div class="available_timelines">
                    {
                        if user_timelines.data.is_some() {
                            html! {
                                for user_timelines
                                    .data
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .map(|timeline| html! {
                                        <TimelineItem
                                            user_timeline={timeline.clone()}
                                            update={update_timeline.clone()}
                                            delete={delete_timeline.clone()}
                                            switch={switch_timeline.clone()}/>
                                    })
                            }
                        } else {
                            html! {"Loading.."}
                        }
                    }
                </div>

            </div>

            <TaskList current_timeline={current_timeline.deref().clone()}/>
        </>
    }
}
