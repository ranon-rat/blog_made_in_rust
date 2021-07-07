use actix_web::{ web};
use crate::blog::types::{PublicationPost};
use crate::blog::database::insert_publication;
//http://127.0.0.1:8080/post?title=hello-world&url_img=discord...&body=how_to_do_...
pub async fn add_to_database(inf: web::Query<PublicationPost>) -> String {
    format!(
        "body: {}\ntitle: {}\nurl image: {}\nhow many rows has been updated: {}",
        inf.0.body.trim(),inf.0.title.trim(), &inf.0.mineature_url,insert_publication(&inf.0)
    )
}