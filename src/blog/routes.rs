use actix_web::{web,HttpServer,App};
use crate::blog::controllers::{add_to_database};
pub async fn setup_routes()-> std::io::Result<()> {
    println!("running server");
    HttpServer::new(|| {
        App::new()
            .route("/post",web::post().to(add_to_database))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}