mod query;
mod mutation;

use std::sync::Arc;

use actix_web::http::header::HeaderMap;
use juniper::{EmptySubscription, RootNode};
use mutation::MutationRoot;
use query::QueryRoot;


pub struct Context {
    pub headers: Arc<HeaderMap>
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
