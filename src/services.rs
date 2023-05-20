use actix_web::{get, HttpResponse, Responder, web, post, delete};
use crate::{persistence::{get_channels, self}, types::{Channel, User, Message, MessageDTO}};

/// catch-all endpoint
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

/// Get the channels
#[get("/channels")]
async fn channels() -> impl Responder {
    let channels = get_channels().await;

    web::Json(channels)
}

/// Create a new channel
#[post("/channels")]
async fn create_channel(channel: web::Json<Channel>) -> impl Responder {
    match persistence::insert_channel(&channel).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

/// delete the channel with the given id
#[delete("/channels/{id}")]
async fn delete_channel(path: web::Path<u32>) -> impl Responder {
    let channel_id = path.into_inner();
    match persistence::delete_channel(channel_id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::NoContent(),
    }
}

/// creates the user with the values needed
#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    match persistence::create_user(&user).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

/// creates the user with the values needed
#[post("/messages")]
async fn create_message(message: web::Json<MessageDTO>) -> impl Responder {
    let msg = Message{
        id: None,
        channel: Channel {
            id: Some(message.channel_id),
            name: "".to_string(),
            users: None,
            messages: None
        },
        user: User {
            id: Some(message.user_id),
            name: "".to_string()
        },
        content: message.content.to_string()
    };
    match persistence::create_message(&msg).await {
        Ok(_) => HttpResponse::Ok(),
        Err(error) => {
            println!("{error}");
            HttpResponse::BadRequest()
        },
    }
}
