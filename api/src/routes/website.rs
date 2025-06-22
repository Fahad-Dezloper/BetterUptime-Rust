use std::{clone, sync::{Arc, Mutex}};

use poem::{
    get, handler, listener::TcpListener, post, web::{Data, Json, Path}, EndpointExt, Route, Server
};
use store::store::Store;

use crate::{request_input::{CreateUserInput, CreateWebsiteInput}, request_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};


#[handler]
pub fn get_websites(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<PgConn>>>) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(id).unwrap();
    Json(GetWebsiteOutput{
        url: website.url
    })
}


#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s): Data<&Arc<Mutex<PgConn>>>) -> Json<CreateWebsiteOutput>  { 
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.create_website(String::from("kasjdkfnsakdfn-4564-ksnjdkfns"), data.url).unwrap();

    let response = CreateWebsiteOutput {
        id: website.id
    };

    Json(response)
}