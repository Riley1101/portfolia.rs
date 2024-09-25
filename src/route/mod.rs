use actix_web::{post, HttpResponse, Responder};

pub mod home;
pub mod api;
pub mod articles;

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
