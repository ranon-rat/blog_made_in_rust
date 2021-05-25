#![feature(proc_macro_hygiene, decl_macro)]

use crate::blog::types::Publication;
use rocket::response::content::Json;

#[post("/users", format = "json")]
pub fn receive_post(post:Json<Publication>){
    println!("{:?}",data);
    println!("{:?}",post);

}