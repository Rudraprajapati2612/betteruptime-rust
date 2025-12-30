use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct CreateWebsitOutput{
    pub id : String
}