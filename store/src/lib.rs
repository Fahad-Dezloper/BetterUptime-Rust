// 1:07:39
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

use crate::config::Config;


pub mod schema;
pub mod config;
pub mod store;
pub mod models;