use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};
use store::store::Store;

use crate::{request_input::{CreateUserInput, CreateWebsiteInput}, request_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};

pub mod request_input;
pub mod request_output;

#[handler]
fn get_websites(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s: Store = Store::default().unwrap();
    let website = s.get_website(id).unwrap();
    Json(GetWebsiteOutput{
        url: website.url
    })
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s = Store::default().unwrap();
    let id = s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput {
        id: user.id
    };

    Json(response)
}

fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SigninOutput> {
    let mut s = Store::default().unwrap();
    let exist = s.sign_in(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: String::from("Fahad")
    };

    Json(response)
}


#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput>  { 
    let mut s = Store::default();
    let id = s.create_website(String::from("kasjdkfnsakdfn-4564-ksnjdkfns"), data.url).unwrap();

    let response = CreateWebsiteOutput {
        id: website.id
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // specify the buisness details of the app
    let app = Route::new()
        .at("/website/:website_id", get(get_websites))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/sigin", post(sign_in));
    // creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}