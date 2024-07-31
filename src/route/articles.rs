use actix_web::{get, web, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use handlebars::Handlebars;
use serde_json::json;

use crate::database::schema::articles::dsl::articles;

use crate::database::models::{Article, DbPool};
use crate::database::schema::articles::published;


#[get("/article/{slug}")] 
async fn article_detail(
    path: web::Path<String>, 
    hb:web::Data<Handlebars<'_>>,
     pool: web::Data<DbPool>,
) -> impl Responder{
    println!("{:#?}", pool);

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let results = articles
        .filter(published.eq(true))
        .limit(5)
        .select(Article::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    for i in results {
        println!("{:#?}", i);
    }


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
