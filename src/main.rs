mod persistence;
mod services;
mod types;

// Internal uses
use persistence::ensure_exists;
use services::{
    channels, create_channel, create_message, create_user, get_channel_messages, get_user_info,
    index,
};

// External uses
use actix_web::{web, App, HttpServer};
use eyre::Result;
use sqlx::PgPool;
use std::net;

/// Main function
#[actix_web::main]
async fn main() -> Result<()> {
    let ip = net::Ipv4Addr::new(0, 0, 0, 0);
    let port = 80;

    println!("Starting server on {ip}:{port}");

    let pool = ensure_exists().await?;

    start_server((ip, port), pool).await?;

    Ok(())
}

/// Start the webserver
async fn start_server<A>(loc: A, pool: PgPool) -> Result<()>
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
    .await?;

    Ok(())
}
