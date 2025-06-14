use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};
use store::Store;

pub mod request_input;
pub mod request_output;

#[handler]
fn get_websites(Path(name): Path<String>) -> String {
    print!("recived request");
    format!("hello: {name}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput>  {
    print!("recived request");
    let url = data.url;
    // persist this in db
    let s = Store{};
    let id = s.create_website();
    s.create_user();

    let response = CreateWebsiteOutput {
        id
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // specify the buisness details of the app
    let app = Route::new()
        .at("/website/:website_id", get(get_websites))
        .at("/website", post(create_website));
    // creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}