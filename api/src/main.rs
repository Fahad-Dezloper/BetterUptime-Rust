use std::{clone, sync::{Arc, Mutex}};

use poem::{
    get, handler, listener::TcpListener, post, web::{Data, Json, Path}, EndpointExt, Route, Server
};
use store::store::Store;

use crate::{request_input::{CreateUserInput, CreateWebsiteInput}, request_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}, routes::{user::{sign_in, sign_up}, website::{create_website, get_websites}}};

pub mod request_input;
pub mod request_output;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let s: Store = Arc::new(Mutex::new(Store::new().unwrap()));
    // specify the buisness details of the app
    let app = Route::new()
        .at("/website/:website_id", get(get_websites))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/sigin", post(sign_in(json, data)))
        .data(s);

    // creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}



// let s = PgConn {
//     conn: String::from("pg")
// };
// let arced_s = Arc::new(s);