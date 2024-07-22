use actix_web::{get, web, Responder};
use handlebars::Handlebars;
use serde_json::json;

#[get("/article/{slug}")]
pub async fn article_detail(path: web::Path<String>, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",
        "nav-aside":"partials/nav-aside",

        "slug": *path,
    });
    let body = hb.render("article-detail", &data).unwrap();
    web::Html::new(body)
}
