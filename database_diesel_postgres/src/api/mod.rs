use graphql_api as query;
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult, EmptyMutation, EmptySubscription, RootNode, FieldError};
use crate::schema::*;
use crate::table::{
    timelines::*,
    users::*,
    timelines_users::*,
    events::*,
    sub_events::*,
};

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}

pub struct Database;
impl juniper::Context for Database {}
impl Database {
    pub fn new() -> Self {
        Self {}
    }

    pub fn userdata(
        &self, 
        id: i32, 
        hashed_password: String
    ) -> FieldResult<query::Userdata> {
        let conn = crate::establish_connection();

        let user = users::table
            .filter(users::dsl::id.eq(id))
            .first::<User>(&conn)?;

        if user.hashed_password != hashed_password {
            return Err(FieldError::from("Wrong password"));
        }

        let mut userdata = query::Userdata { id, timelines: Vec::new() };

        let properties_timelines = TimelinesUsers::belonging_to(&user)
            .inner_join(timelines::table)
            .load::<(TimelinesUsers, Timeline)>(&conn)?;

        for (props, timeline) in properties_timelines.iter() {
            let events = Event::belonging_to(timeline).load::<Event>(&conn)?;
            let mut timeline = query::Timeline { 
                id: timeline.id,
                color: props.color.clone(),
                events: Vec::new(),
                tasks: Vec::new(),
            };

            for event in events.iter() {
                match event.done {
                    Some(done) => {
                        let sub_events = SubEvent::belonging_to(event).load::<SubEvent>(&conn)?;
                        let mut task = query::Task {
                            id: event.id,
                            timeline_id: timeline.id,
                            title: event.title.clone(), 
                            body: event.body.clone(),
                            done,
                            end_time: event.end_time,
                            sub_tasks: Vec::new(),
                        };
                        for sub_event in sub_events.iter() {
                            task.sub_tasks.push(query::SubTask {
                                id: sub_event.id,
                                task_id: event.id,
                                title: sub_event.title.clone(),
                                done: sub_event.done.unwrap(),
                            });
                        }
                        timeline.tasks.push(task);
                    },
                    None => {
                        timeline.events.push(query::Event {
                            id: event.id,
                            timeline_id: timeline.id,
                            title: event.title.clone(), 
                            body: event.body.clone(),
                            start_time: event.start_time.unwrap(),
                            end_time: event.end_time,
                        });
                    }
                }
            }
            userdata.timelines.push(timeline);
        }
        Ok(userdata)
    }
}

pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    pub fn api_version() -> &'static str {
        "0.1"
    }

    pub fn userdata(
        context: &Database, 
        id: i32, 
        hashed_password: String
    ) -> FieldResult<query::Userdata> {
        context.userdata(id, hashed_password)
    }
}

