use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};

#[allow(dead_code)]
pub enum ErrorExample {
    Unauthorized,
    NotFound,
    CommonError
}

impl<S: ScalarValue> IntoFieldError<S> for ErrorExample {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            ErrorExample::Unauthorized => FieldError::new(
                "Error",
                graphql_value!({
                    "type": "UNAUTHORIZED",
                    "map":"EE-401",
                }),
            ),
            ErrorExample::NotFound => FieldError::new(
                "Not Found",
                graphql_value!({
                    "type": "NOT_FOUND",
                    "map":"EE-404",
                }),
            ),
            ErrorExample::CommonError => FieldError::new(
                "Error",
                graphql_value!({
                    "type": "COMMON_ERROR",
                    "map":"EE-500",
                }),
            ),
        }
    }
}
