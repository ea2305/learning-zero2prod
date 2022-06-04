use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(from: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
