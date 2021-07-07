use actix_web::{web,HttpServer,App};
use crate::blog::controllers::*;
pub async fn setup_routes()-> std::io::Result<()> {
    println!("running server");
    HttpServer::new(|| {
        App::new()
            .route("/post",web::post().to(add_to_database))
            .route("/public/{filename:.*}", web::get().to(public_files))
            .route("/admin/new-post",web::get().to(send_post_file))
            .route("/publication",web::get().to(load_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}