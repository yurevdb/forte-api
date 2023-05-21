mod services;
mod persistence;
mod types;

use std::net;

// Internal uses
use services::{index, channels, create_channel, delete_channel, create_user, create_message, get_channel_messages, get_user_info};
use persistence::ensure_exists;

// External uses
use actix_web::{App, HttpServer,};

/// Main function
#[actix_web::main]
async fn main() {

    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;

    println!("Starting server on {ip}:{port}");

    // Ensure the database exists and is created
    ensure_exists().await.unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    start_server((ip, port)).await.unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });
}

/// Start the webserver
async fn start_server<A>(loc: A) -> std::io::Result<()> 
    where A: net::ToSocketAddrs {
    // Start the webserver
    HttpServer::new(|| {
        App::new()
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
