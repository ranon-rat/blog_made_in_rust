use crate::blog::types::{PublicationPost, QueryPublication};
use actix_files::NamedFile;
use actix_web::{
    web::{Form, Query},
    HttpRequest, Result,HttpResponse
};
use askama::Template;
use std::path::PathBuf;

use crate::blog::database::{get_publication, insert_publication};
//http://127.0.0.1:8080/post?title=hello-world&body=yes_no&mineature_url=discord.com/1234..
pub async fn add_to_database(inf: Form<PublicationPost>) -> String {
    format!(
        "body: {}\ntitle: {}\nurl image: {}\nhow many rows has been updated: {}",
        inf.0.body.trim(),
        inf.0.title.trim(),
        &inf.0.mineature_url,
        insert_publication(&inf.0)
    )
}
pub async fn public_files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf =
        PathBuf::from(String::from("src/public/") + req.match_info().query("filename"));

    Ok(NamedFile::open(path)?) //.unwrap_or(NamedFile::open("src/public/sorry.txt").unwrap())
}
#[allow(dead_code)]
pub async fn load_post(inf: Query<QueryPublication>) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(get_publication(inf.0).render().unwrap())
}
pub async fn send_post_file() -> Result<NamedFile> {
    Ok(NamedFile::open("src/view/post.html")?)
}
