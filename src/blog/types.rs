use serde::Deserialize;
#[derive(Deserialize,Clone)]

pub struct PublicationPost{
   pub title:String,
   pub mineature_url:String,
   pub body:String,
}
