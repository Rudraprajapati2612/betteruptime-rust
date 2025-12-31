use std::{ sync::{Arc, Mutex}};

use poem ::{ handler, web::{Data, Json, Path}};
use store::store::Store;
use crate::{request_input::{CreateWebsiteInput}, request_output::{CreateWebsitOutput, GetWebsiteOutput}};
#[handler]
pub fn get_website(Path(website_id): Path<String>,Data(s):Data<&Arc<Mutex<Store>>>) ->Json<GetWebsiteOutput>{
   
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.get_website(website_id).unwrap();

    let response  = GetWebsiteOutput{
        url : website.url
    };

    return Json(response)
}

#[handler]
pub fn create_website(Json(data):Json<CreateWebsiteInput>,Data(s):Data<&Arc<Mutex<Store>>>) ->Json<CreateWebsitOutput>{
    // let url  = data.url;
    // store data in database 
    
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.create_website(String::from("1aa28a60-2f3f-44e6-8913-30b86c62472c"),data.url).unwrap();
    let response = CreateWebsitOutput{
        id : website.id
    };
    
    return Json(response);
}