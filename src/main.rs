mod services;
mod persistence;
mod types;

// Internal uses
use services::{index, channels, create_channel, delete_channel, create_user, create_message};
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
            .service(channels)
            .service(create_channel)
            .service(delete_channel)
            .service(create_user)
            .service(create_message)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
