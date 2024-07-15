use serde_json::json;
use handlebars::Handlebars;
use actix_web::{
    get,
    web, Responder, 
    HttpResponse
};

#[get("/css/{filename:.*}")]
async fn home(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    HttpResponse::Ok().body("Hello CSS!")
}
