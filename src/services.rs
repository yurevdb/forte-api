use crate::{
    persistence::{self, get_channels},
    types::{Channel, Message, User},
};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

/// catch-all endpoint
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

/// Get the channels
#[get("/channels")]
async fn channels(pool: web::Data<PgPool>) -> impl Responder {
    match get_channels(&pool).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

/// Create a new channel
#[post("/channels")]
async fn create_channel(pool: web::Data<PgPool>, channel: web::Json<Channel>) -> impl Responder {
    match persistence::insert_channel(&pool, &channel).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::BadRequest(),
    }
}

/// delete the channel with the given id
#[delete("/channels/{id}")]
async fn delete_channel(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let channel_id = path.into_inner();
    match persistence::delete_channel(&pool, channel_id).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::NoContent(),
    }
}

/// creates the user with the values needed
#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    match persistence::create_user(&pool, &user).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

/// creates the user with the values needed
#[post("/messages")]
async fn create_message(pool: web::Data<PgPool>, message: web::Json<Message>) -> impl Responder {
    match persistence::create_message(&pool, &message).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

/// get messages from a channel
#[get("/channels/{id}/messages")]
async fn get_channel_messages(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let channel_id = path.into_inner();

    match persistence::get_channel_messages(&pool, channel_id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/users/{id}")]
async fn get_user_info(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();

    match persistence::get_user_info(&pool, user_id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
