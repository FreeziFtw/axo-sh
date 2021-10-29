use actix_web::{get, post, Responder, HttpResponse};

#[post("/")]
pub async fn add_url() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/{id}/")]
pub async fn get_url_by_id() -> impl Responder {
    HttpResponse::Ok()
}