mod services;
mod persistence;

// Internal uses
use services::index;
use persistence::ensure_exists;

// External uses
use actix_web::{App, HttpServer,};

/// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure the database exists and is created
    ensure_exists().await;

    // Start the webserver
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
