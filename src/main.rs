mod controllers;

use actix_web::{App, HttpServer};
use tokio::sync::Mutex;

struct AppState {
    db: Mutex<sqlx::SqlitePool>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controllers::auth::signup)
            .service(controllers::auth::signin)
            .service(controllers::me::get_profile)
            .service(controllers::me::update_profile)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    println!("Server running at http://127.0.0.1:8080");
    Ok(())
}
