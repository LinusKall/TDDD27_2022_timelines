mod graphql;
mod db;

use entity::async_graphql;
use graphql::schema::{build_schema, AppSchema};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    get,
    http::header,
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result
};

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello from backend!")
}

async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/api/graphql")
            .subscription_endpoint("/api/graphql"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let schema = build_schema().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(test)
            .service(
                web::resource("/api/graphql")
                    .route(web::post().to(graphql_handler))
                    .route(web::get().to(graphql_playground)),
            )
            // .service(web::resource("/api/playground").route(web::get().to(graphql_playground)))
            // .service(web::resource("/api/graphiql").route(web::get().to(graphql_handler)))
            .service(fs::Files::new("/", "../frontend_yew/dist/").index_file("index.html"))
            .service(
                fs::Files::new("/images/", "../frontend_yew/dist/images/").show_files_listing(),
            )
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
