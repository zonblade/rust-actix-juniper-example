use juniper::{graphql_object, FieldResult};

use super::Context;

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    async fn test() -> FieldResult<String> {
        Ok("test".to_owned())
    }
}
