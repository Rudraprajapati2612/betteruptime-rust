
use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid; 
use chrono::{NaiveDateTime, Utc};
#[derive(Queryable,Insertable,Selectable)]
#[diesel(table_name= crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]


pub struct Website{
   pub  id: String,
   pub  url : String,
   pub  userId: String,
   pub  time_added:NaiveDateTime
}

impl Store{
    pub fn create_website(&mut self , user_id:String, url:String)->Result<Website,diesel::result::Error>{
            let id = Uuid::new_v4();
            let website = Website{
                id : id.to_string(),
                url : url ,
                userId:user_id,
                time_added:Utc::now().naive_utc()
            };

            let web  = diesel::insert_into(crate::schema::website::table)
            .values(&website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;


            Ok(web)
    }
    pub fn get_website(&mut self,input_id:String)->Result<Website,diesel::result::Error>{
        use crate::schema::website::dsl::*;
        let website_result = website.filter(id.eq(input_id))
        .select(Website::as_select())
        .first(&mut self.conn)?;

        Ok(website_result)
    }
}