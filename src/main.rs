mod persistence;
mod services;
mod types;

use std::net;

// Internal uses
use persistence::ensure_exists;
use services::{
    channels, create_channel, create_message, create_user, get_channel_messages, get_user_info,
    index,
};

// External uses
use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;

/// Main function
#[actix_web::main]
async fn main() {
    let ip = net::Ipv4Addr::new(0, 0, 0, 0);
    let port = 8080;

    println!("Starting server on {ip}:{port}");

    // Ensure the database exists and is created
    let pool = ensure_exists().await.unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    start_server((ip, port), pool).await.unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });
}

/// Start the webserver
async fn start_server<A>(loc: A, pool: SqlitePool) -> std::io::Result<()>
where
    A: net::ToSocketAddrs,
{
    // Start the webserver
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(channels)
            .service(create_channel)
            .service(create_user)
            .service(create_message)
            .service(get_channel_messages)
            .service(get_user_info)
    })
    .bind(loc)?
    .run()
    .await
}
