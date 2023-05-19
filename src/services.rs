use actix_web::{get, HttpResponse, Responder, Result, web, post};
use crate::{persistence::{get_channels, self}, types::Channel};

/// catch-all endpoint
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

/// Get the channels
#[get("/channels")]
async fn channels() -> Result<impl Responder> {
    let channels = get_channels().await;

    Ok(web::Json(channels))
}

/// Create a new channel
#[post("/channels")]
async fn create_channel(channel: web::Json<Channel>) -> Result<impl Responder> {
    persistence::insert_channel(&channel).await.unwrap();

    Ok(web::Json(channel))
}
