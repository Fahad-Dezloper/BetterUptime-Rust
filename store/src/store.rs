use diesel::{Connection, PgConnection};

use crate::config::Config;

pub struct Store {
    pub conn: PgConnection
}

impl Store {
    pub fn default() -> Self {
        let config = Config::default();
        let conn = PgConnection::establish(&config.database_url)
            .expect("Error connecting to database");
        Self { conn }
    }
}