use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, post, web::Path, EndpointExt, Route, Server
};

pub mod request_input;
pub mod request_output;

#[handler]
fn get_websites(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn create_website() -> String  {
 
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