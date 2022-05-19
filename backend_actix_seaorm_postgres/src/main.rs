mod db;
mod graphql;
mod routes;

use graphql::schema::build_schema;
use routes::{gql, test};

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    guard,
    http::header,
    middleware,
    web::{self, Data},
    App, HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let schema = build_schema().await;

    HttpServer::new(move || {
        App::new()
            // Add data.
            .app_data(Data::new(schema.clone()))
            // Add CORS.
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            // Add middleware.
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // Add routes.
            .service(test)
            .service(
                web::resource("/api/graphql")
                    .route(web::post().to(gql::requests))
                    .route(web::get().to(gql::playground)),
            )
            .service(
                web::resource("/api/graphql")
                    .guard(guard::Post())
                    .guard(guard::Header("upgrade", "websocket"))
                    .route(web::post().to(gql::websocket)),
            )
            .service(fs::Files::new("/", "../frontend_yew/dist/").index_file("index.html"))
            .service(
                fs::Files::new("/images/", "../frontend_yew/dist/images/").show_files_listing(),
            )
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
