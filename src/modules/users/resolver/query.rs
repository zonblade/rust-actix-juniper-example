use juniper::graphql_object;

use crate::modules::users::{gql::error::ErrorExample, services::ServiceExample};

use super::Context;

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    async fn test_ok() -> Result<String, ErrorExample> {
        let result = ServiceExample::test().await;
        match result {
            Ok(value) => Ok(value),
            Err(err) => match err {
                1 => Err(ErrorExample::CommonError),
                _ => Err(ErrorExample::CommonError),
            },
        }
    }

    async fn test_err() -> Result<String, ErrorExample> {
        let result = ServiceExample::test_err().await;
        match result {
            Ok(value) => Ok(value),
            Err(err) => match err {
                1 => Err(ErrorExample::NotFound),
                _ => Err(ErrorExample::CommonError),
            },
        }
    }
}
