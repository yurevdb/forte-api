use actix_web::{get, HttpResponse, Responder};

/// catch-all endpoint
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}


