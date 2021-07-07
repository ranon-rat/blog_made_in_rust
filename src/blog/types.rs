
use serde::Deserialize;
use askama::Template;

#[derive(Deserialize,Clone,Template)]
#[template(path= "templatePublication.html" )]
pub struct PublicationPost{
   pub title:String,
   pub mineature_url:String,
   pub body:String,
}
#[derive(Deserialize,Clone)]
pub struct QueryPublication{
   pub id:i32,
   pub name:String,
}

