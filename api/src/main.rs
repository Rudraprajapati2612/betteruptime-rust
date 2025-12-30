use std::{io::Error};

use poem ::{Route, Server, get, handler,  listener::TcpListener, post, web::{Json, Path}};

use crate::{request_input::{ CreateWebsiteInput}, request_output::CreateWebsitOutput};
pub mod request_input;
use store::store::Store;
pub mod request_output;
#[handler]
fn get_website(Path(website_id): Path<String>) ->String{
   
    format!("hello: {}",website_id)
}

#[handler]
fn create_website(Json(data):Json<CreateWebsiteInput>) ->Json<CreateWebsitOutput>{
    // let url  = data.url;
    // store data in database 
    let s = Store::default();
    let id = s.create_website();
    let response = CreateWebsitOutput{
        id 
    };
    
    return Json(response);
}
 
#[tokio::main]
async fn main() -> Result<(),Error>{
    
    // Routing of the app 
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website));
    
    
    // Create and run the http server 
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await.unwrap() ; 
    Ok(())
}

