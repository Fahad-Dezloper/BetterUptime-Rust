// 1:07:39
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

use crate::config::Config;


pub mod schema;
pub mod config;

pub struct Store {
    pub conn: PgConnection
}

impl Default for Store {
    fn default() -> Self {
        let config = Config::default();
        let conn = PgConnection::establish(&config.database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url));
        Ok(Self { 
            conn 
        })
    }
}

impl Store {
    pub fn create_user(&self) {
        print!("Create User Called");
        self.conn.execute("INSERT INTO xyz");
    }

    pub fn create_website(&self) -> String {
        String::from("1")
    }
}