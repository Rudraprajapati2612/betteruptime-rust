use std::{io::Error};

use poem ::{Route, Server, get, handler,  listener::TcpListener, post, web::{Json, Path}};

use crate::{request_input::{CreateWebsiteInput,  SignUpInput}, request_output::{CreateWebsitOutput, GetWebsiteOutput, SignInOutput, SignUpOutput}};
pub mod request_input;
use store::store::Store;
pub mod request_output;

#[handler]
fn sign_up(Json(data):Json<SignUpInput>)->Json<SignUpOutput>{
    // take input the param  and check it in the db 
    let mut s = Store::default().unwrap();
    let id = s.sign_up(data.username, data.password).unwrap();

    let response = SignUpOutput{
         id : id
    };

    return Json(response);
}


#[handler]
fn sign_in(Json(data):Json<SignUpInput>)->Json<SignInOutput>{
    // take input the param  and check it in the db 
    let mut s = Store::default().unwrap();
    let exist = s.sign_in(data.username, data.password).unwrap();
     
    let response = SignInOutput{
        jwt : String::from("Rudra")
    };

    return Json(response);
}


#[handler]
fn get_website(Path(website_id): Path<String>) ->Json<GetWebsiteOutput>{
   
    let mut  s = Store::default().unwrap();
    let website = s.get_website(website_id).unwrap();

    let response  = GetWebsiteOutput{
        url : website.url
    };

    return Json(response)
}

#[handler]
fn create_website(Json(data):Json<CreateWebsiteInput>) ->Json<CreateWebsitOutput>{
    // let url  = data.url;
    // store data in database 
    
    let mut s = Store::default().unwrap();
    let website = s.create_website(String::from("1aa28a60-2f3f-44e6-8913-30b86c62472c"),data.url).unwrap();
    let response = CreateWebsitOutput{
        id : website.id
    };
    
    return Json(response);
}
 
#[tokio::main]
async fn main() -> Result<(),Error>{
    
    // Routing of the app 
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website))
    .at("/user/signup",post(sign_up))
    .at("/user/signin",post(sign_in));
    
    
    // Create and run the http server 
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await.unwrap() ; 
    Ok(())
}

