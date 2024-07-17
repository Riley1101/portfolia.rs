use serde_json::json;
use handlebars::Handlebars;
use actix_web::{
    get,
    web, Responder, 
};


#[get("/")]
async fn home(hb: web::Data<Handlebars<'_>>) -> impl Responder {
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
async fn about(hb: web::Data<Handlebars<'_>>) -> impl Responder {
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
async fn articles(hb: web::Data<Handlebars<'_>>) -> impl Responder {
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



#[get("/snippets")]
async fn snippets(hb: web::Data<Handlebars<'_>>) -> impl Responder {
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

#[get("/videos")]
async fn videos(hb: web::Data<Handlebars<'_>>) -> impl Responder {
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
