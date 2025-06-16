use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    Self {
        database_url
    }   
}
}
