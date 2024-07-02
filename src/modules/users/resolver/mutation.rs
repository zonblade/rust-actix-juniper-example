use juniper::{graphql_object, FieldResult};

use super::Context;

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn test() -> FieldResult<String> {
        Ok("test".to_owned())
    }
}
