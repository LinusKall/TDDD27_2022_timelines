use crate::schema::*;
use crate::table::{events::*, sub_tasks::*, tasks::*, timelines::*, timelines_users::*, users::*};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use graphql_api as query;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldError, FieldResult, RootNode,
};
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
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
        hashed_password: String,
    ) -> FieldResult<query::Userdata> {
        context.userdata(id, hashed_password)
    }
}

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl juniper::Context for Database {}
impl Database {
    pub fn new() -> Self {
        let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL is not definied");
        let manager = ConnectionManager::<PgConnection>::new(conn_spec);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create db connection pool.");
        Self { pool }
    }

    pub fn userdata(&self, id: i32, hashed_password: String) -> FieldResult<query::Userdata> {
        let conn = self.pool.get()?;

        let user = users::table
            .filter(users::dsl::id.eq(id))
            .first::<User>(&conn)?;

        if user.hashed_password != hashed_password {
            return Err(FieldError::from("Wrong password"));
        }

        let mut userdata = query::Userdata {
            id,
            timelines: Vec::new(),
        };

        let properties_timelines = TimelinesUsers::belonging_to(&user)
            .inner_join(timelines::table)
            .load::<(TimelinesUsers, Timeline)>(&conn)?;

        for (props, timeline) in properties_timelines.iter() {
            let events = Event::belonging_to(timeline).load::<Event>(&conn)?;
            let tasks = Task::belonging_to(timeline).load::<Task>(&conn)?;

            let mut timeline = query::Timeline {
                id: timeline.id,
                color: props.color.clone(),
                events: Vec::new(),
                tasks: Vec::new(),
            };

            for event in events.iter() {
                timeline.events.push(query::Event {
                    id: event.id,
                    timeline_id: timeline.id,
                    title: event.title.clone(),
                    body: event.body.clone(),
                    start_time: event.start_time,
                    end_time: event.end_time,
                });
            }

            for task in tasks.iter() {
                let sub_tasks = SubTask::belonging_to(task)
                    .load::<SubTask>(&conn)?
                    .iter()
                    .map(|sub_task| query::SubTask {
                        id: sub_task.id,
                        task_id: task.id,
                        title: sub_task.title.clone(),
                        done: sub_task.done,
                    })
                    .collect();

                timeline.tasks.push(query::Task {
                    id: task.id,
                    timeline_id: timeline.id,
                    title: task.title.clone(),
                    body: task.body.clone(),
                    done: task.done,
                    sub_tasks,
                    end_time: task.end_time,
                });
            }
            userdata.timelines.push(timeline);
        }
        Ok(userdata)
    }
}
