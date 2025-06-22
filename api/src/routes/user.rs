use std::{clone, sync::{Arc, Mutex}};

use poem::{
    get, handler, listener::TcpListener, post, web::{Data, Json, Path}, EndpointExt, Route, Server
};
use store::store::Store;

use crate::{request_input::{CreateUserInput, CreateWebsiteInput}, request_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};



#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<PgConn>>>) -> Json<CreateUserOutput> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput {
        id: user.id
    };

    Json(response)
}

pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<PgConn>>>) -> Json<SigninOutput> {
    let mut locked_s = s.lock().unwrap();
    let exist = locked_s.sign_in(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: String::from("Fahad")
    };

    Json(response)
}
