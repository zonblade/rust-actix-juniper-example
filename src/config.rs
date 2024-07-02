use std::env;

use mini_config::Configure;

#[derive(Debug, Clone, Configure)]
pub enum Config {
    DatabaseURI
}

pub fn init(){
    dotenv::dotenv().ok();
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    Config::DatabaseURI.set(&mongo_uri);
}