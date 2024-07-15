use actix_web::{post, HttpResponse, Responder};

pub mod home;
pub mod static_file;

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
