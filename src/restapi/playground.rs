use actix_web::Responder;
use actix_web_lab::respond::Html;
use juniper::http::playground::playground_source;

pub async fn graphql_playground() -> impl Responder {
    Html(playground_source("", None))
}
