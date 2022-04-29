mod graphql;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    get,
    http::header,
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use db::api::{schema, Database};
use graphql::*;

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello from backend!")
}

// https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let db_context = Database::new();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .app_data(Data::new(db_context.clone()))
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
