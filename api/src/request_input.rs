use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct CreateWebsiteInput{
    pub url : String
}

#[derive(Serialize,Deserialize)]
pub struct SignUpInput{
    pub username : String,
    pub password : String
}