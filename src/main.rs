use std::io;

use actix_web::{web, App, HttpServer};
use config::Config;

pub mod config;
pub mod library;
pub mod modules;
pub mod restapi;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();
    config::init();

    let db_uri = Config::DatabaseURI.get_str();
    let db_client = mongodb::Client::with_uri_str(db_uri)
        .await
        .expect("Database Failed to Connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .service(web::resource("/playground")
                .route(web::get().to(restapi::playground::graphql_playground)
            ))
            .service(web::scope("/gql")
                .service(web::resource("/contoh")
                    .route(web::post().to(modules::users::route::graphql))
                )
            )
    })
    .workers(4)
    .bind(("0.0.0.0", 2222))?
    .run()
    .await
}
