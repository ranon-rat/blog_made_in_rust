
mod blog;




#[actix_web::main]
async fn main() -> std::io::Result<()> {

    blog::routes::setup_routes().await
}
