use crate::markdown_content::renderer::render_markdown;
use actix_web::{get, web, Responder};
use handlebars::to_json;
use handlebars::Handlebars;
use serde_json::json;

use crate::database::models::{Article, ArticleCRUD, DbPool};

#[get("/articles/{slug}")]
async fn article_detail(
    path: web::Path<String>,
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let article = Article::get_article_by_slug(conn, path.clone());

    let html_string = render_markdown(article.body.clone());

    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",
        "nav-aside":"partials/nav-aside",
        "slug": *path,
        "article": to_json(article),
        "block": html_string,
    });

    let body = hb.render("article-detail", &data).unwrap();
    web::Html::new(body)
}

#[get("/articles")]
async fn articles(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let articles = Article::get_all_articles(conn);

    let data = json!({
        "name": "Handlebars",
        "layout":"partials/layout",
        "header":"partials/header",
        "footer":"partials/footer",
        "nav-aside":"partials/nav-aside",
        "categories":"partials/categories",
        "articles": to_json(articles),
    });

    let body = hb.render("articles", &data).unwrap();
    web::Html::new(body)
}
