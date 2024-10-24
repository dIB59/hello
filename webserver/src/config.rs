use std::env;

use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub typesense_url: String,
    pub typesense_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let typesense_url  = env::var("TYPESENSE_URL").expect("TYPESENSE_URL must be set");
        let typesense_api_key = env::var("TYPESENSE_API_KEY").expect("TYPESENSE_API_KEY must be set");
        Config { database_url , typesense_url ,typesense_api_key }
    }
}
