use serde_json::json;
use handlebars::Handlebars;
use std::sync::Mutex;
use actix_web::{
    get,
    web, Responder, 
};

use crate::sanity::client::{ SanityClient , Client};

#[get("/")]
async fn home(hb: web::Data<Handlebars<'_>>,sn: web::Data<Mutex<SanityClient>>) -> impl Responder {
    sn.lock().unwrap().query();
    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",
        "nav-aside":"partials/nav-aside",
        "home-intro":"partials/home-intro",
        "latest-articles":"partials/latest-articles",
        "projects":"partials/projects",
        "newsletter":"partials/newsletter",
        "article-series":"partials/article-series",
    });
    let body = hb.render("index", &data).unwrap();
    web::Html::new(body)
}

#[get("/about")]
async fn about(hb: web::Data<Handlebars<'_>>,sn: web::Data<Mutex<SanityClient>>) -> impl Responder {
    sn.lock().unwrap().query();
    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",

        "nav-aside":"partials/nav-aside",

    });
    let body = hb.render("about", &data).unwrap();
    web::Html::new(body)
}

#[get("/articles")]
async fn articles(hb: web::Data<Handlebars<'_>>,sn: web::Data<Mutex<SanityClient>>) -> impl Responder {
    sn.lock().unwrap().query();
    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",
        "nav-aside":"partials/nav-aside",
        "categories":"partials/categories",
    });
    let body = hb.render("articles", &data).unwrap();
    web::Html::new(body)
}



#[get("/snippets")]
async fn snippets(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",
        "nav-aside":"partials/nav-aside",
    });
    let body = hb.render("snippets", &data).unwrap();
    web::Html::new(body)
}

#[get("/videos")]
async fn videos(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",
        "nav-aside":"partials/nav-aside",
    });
    let body = hb.render("videos", &data).unwrap();
    web::Html::new(body)
}
