use db::api::*;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    http::header,
    middleware,
    web::{self, Data},
    get, Responder, App, Error, HttpResponse, HttpServer,
};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}

async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello from backend!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(schema()))
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
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
            .service(fs::Files::new("/", "../frontend_yew/dist/").index_file("index.html"))
            .service(
                fs::Files::new("/images/", "../frontend_yew/dist/images/").show_files_listing(),
            )
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
