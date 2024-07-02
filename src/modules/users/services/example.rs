use super::ServiceExample;


impl ServiceExample {
    pub async fn test() -> Result<String, i32> {
        Ok("test".to_owned())
    }
    pub async fn test_err() -> Result<String, i32> {
        Err(1)
    }
}