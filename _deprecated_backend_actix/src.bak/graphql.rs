use actix_web::{web, Error, HttpResponse};
use db::api::{Database, Schema};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

pub async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

pub async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

pub async fn graphql_route(
    context: web::Data<Database>,
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &*context, req, payload).await
}
