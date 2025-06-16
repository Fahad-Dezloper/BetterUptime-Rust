use crate::{schema::user::password, store::Store};
use diesel::prelude::*;
#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: String,
    username: String,
    password: String
}

impl Store {
    pub fn sign_up(&self, username: String, password: String) {
        
    }

    pub fn sign_in(&self, username: String, password: String) {

    }
}