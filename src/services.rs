use actix_web::{get, HttpResponse, Responder, web, post, delete};
use sqlx::SqlitePool;
use crate::{persistence::{get_channels, self}, types::{Channel, User, Message}};

/// catch-all endpoint
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

/// Get the channels
#[get("/channels")]
async fn channels(pool: web::Data<SqlitePool>) -> impl Responder {
    let channels = get_channels(&pool).await;

    web::Json(channels)
}

/// Create a new channel
#[post("/channels")]
async fn create_channel(pool: web::Data<SqlitePool>, channel: web::Json<Channel>) -> impl Responder {
    match persistence::insert_channel(&pool, &channel).await {
        Ok(result) => HttpResponse::Created().json(result.last_insert_rowid()),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

/// delete the channel with the given id
#[delete("/channels/{id}")]
async fn delete_channel(pool: web::Data<SqlitePool>, path: web::Path<u32>) -> impl Responder {
    let channel_id = path.into_inner();
    match persistence::delete_channel(&pool, channel_id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::NoContent(),
    }
}

/// creates the user with the values needed
#[post("/users")]
async fn create_user(pool: web::Data<SqlitePool>, user: web::Json<User>) -> impl Responder {
    match persistence::create_user(&pool, &user).await {
        Ok(result) => {
            HttpResponse::Created().json(result.last_insert_rowid())
        },
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

/// creates the user with the values needed
#[post("/messages")]
async fn create_message(pool: web::Data<SqlitePool>, message: web::Json<Message>) -> impl Responder {
    match persistence::create_message(&pool, &message).await {
        Ok(result) => HttpResponse::Created().json(result.last_insert_rowid()),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

/// get messages from a channel
#[get("/channels/{id}/messages")]
async fn get_channel_messages(pool: web::Data<SqlitePool>, path: web::Path<u32>) -> impl Responder {
    let channel_id = path.into_inner();

    let messages = persistence::get_channel_messages(&pool, channel_id).await;

    web::Json(messages)
}

#[get("/users/{id}")]
async fn get_user_info(pool: web::Data<SqlitePool>, path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();

    let user_info = persistence::get_user_info(&pool, user_id).await;

    web::Json(user_info)
}
