use actix_web::{get, web, Responder};
use handlebars::Handlebars;
use serde_json::json;

#[get("/articles/{slug}")]
pub async fn article_detail(path: web::Path<(u32, String)>, hb: web::Data<Handlebars<'_>>) -> impl Responder {
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
    let body = hb.render("articles/intro-to-react", &data).unwrap();
    web::Html::new(body)
}
