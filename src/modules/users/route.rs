use actix_web::{web, HttpResponse, Responder};
use juniper::http::GraphQLRequest;

use super::resolver::{Context, Schema};

pub async fn graphql(
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>
) -> impl Responder {

    let ctx = Context {};
    let user = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(user)
}
