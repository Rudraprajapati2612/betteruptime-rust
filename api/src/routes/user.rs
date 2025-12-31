use std::{ sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};
use store::{ schema::user, store::Store};
use poem ::{ Error, handler, http::StatusCode, web::{Data, Json}};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{request_input::{  SignUpInput}, request_output::{ SignInOutput, SignUpOutput}};
#[derive(Debug,Serialize,Deserialize)]
 struct Calims{
    sub :String,
    exp : usize
}

#[handler]
pub fn sign_up(Json(data):Json<SignUpInput>,Data(s):Data<&Arc<Mutex<Store>>>)->Result<Json<SignUpOutput>,Error>{
    // take input the param  and check it in the db 
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).map_err(|_|Error::from_status(StatusCode::CONFLICT))?;

    let response = SignUpOutput{
         id 
    };

    return Ok(Json(response));
}


#[handler]
pub fn sign_in(Json(data):Json<SignUpInput>,Data(s):Data<&Arc<Mutex<Store>>>)->Result<Json<SignInOutput>,Error>{
    // take input the param  and check it in the db 
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_in(data.username, data.password);

    match  user_id {
        Ok(user_id)=>{

            let my_claims = Calims{
                sub :user_id,
                exp : 1111111111111111
            };
            
            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).map_err(|_|Error::from_status(StatusCode::UNAUTHORIZED))?;
            
            let response = SignInOutput{
                jwt : token
            };
             Ok(Json(response))
        }
        Err(_)=> Err(Error::from_status(StatusCode::UNAUTHORIZED))
    }
         
    
}
