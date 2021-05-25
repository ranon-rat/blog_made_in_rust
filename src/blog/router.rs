
#[macro_use] extern crate rocket;
use rocket::Rocket;
mod controllers;
pub fn setup_router()->Rocket{
   
rocket::ignite().mount("/", routes![controllers::receive_post])
}  