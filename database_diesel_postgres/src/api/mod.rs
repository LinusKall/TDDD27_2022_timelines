pub mod client_server;
pub mod server_database;

use crate::schema::*;
use crate::table::timelines::*;
use diesel::prelude::*;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

pub struct Database;
impl juniper::Context for Database {}
impl Database {
    pub fn new() -> Self {
        Self {}
    }
    pub fn timelines(&self) -> Vec<Timeline> {
        let connection = crate::establish_connection();

        timelines::dsl::timelines
            .load::<Timeline>(&connection)
            .expect("Failed to load timelines")
    }
}

pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    pub fn api_version() -> &'static str {
        "0.1"
    }

    pub fn timelines(context: &Database) -> Vec<Timeline> {
        context.timelines()
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}
