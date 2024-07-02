use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use juniper::http::GraphQLRequest;

use super::resolver::{Context, Schema};

pub async fn graphql(
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
    req: actix_web::HttpRequest
) -> impl Responder {
    let headers = Arc::new(req.headers().clone());
    let ctx = Context {
        headers
    };
    let user = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(user)
}
