use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
pub struct CreateWebsitOutput{
    pub id : String
}
#[derive(Serialize,Deserialize)]
pub struct SignUpOutput{
    pub id:String
}

#[derive(Serialize,Deserialize)]
pub struct SignInOutput{
    pub jwt : String
}
#[derive(Serialize,Deserialize)]
pub struct GetWebsiteOutput{
    pub url : String
}