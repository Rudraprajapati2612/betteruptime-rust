use std::{io::Error, sync::{Arc, Mutex}};

use poem ::{EndpointExt, Route, Server, get,  listener::TcpListener, post};

use crate::{ routes::{user::{sign_in, sign_up}, website::{create_website, get_website}}};
pub mod request_input;
pub mod routes;
use store::store::Store;
pub mod request_output;

#[tokio::main(flavor = "multi_thread" )]
async fn main() -> Result<(),Error>{
    
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    // Routing of the app 
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website))
    .at("/user/signup",post(sign_up))
    .at("/user/signin",post(sign_in))
    .data(s);
    
    
    // Create and run the http server 
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await.unwrap() ; 
    Ok(())
}

