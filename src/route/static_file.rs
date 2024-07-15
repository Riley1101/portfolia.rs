use handlebars::Handlebars;
use actix_web::{
    get, web, HttpResponse, Responder 
};

#[get("/css/{filename:.*}")]
async fn  css(_: web::Data<Handlebars<'_>>) -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
