mod route;
use actix_web::middleware::Logger;
use actix_web::{ App, HttpServer, web, middleware};
use env_logger::Env;
use handlebars::{DirectorySourceOptions, Handlebars};
use route::echo;
use route::home::{ home , about, articles, snippets, videos};
use route::articles::article_detail;
use actix_files as fs;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let conn = establish_connection();

    let mut handlebars = Handlebars::new();

    handlebars.set_dev_mode(true);
    handlebars
        .register_templates_directory(
            "templates",
            DirectorySourceOptions {
                tpl_extension: ".html".to_owned(),
                hidden: false,
                temporary: false,
            },
        )
        .unwrap();

    handlebars.register_partial("header", "{{ @partials/header }}").unwrap();
    handlebars.register_partial("footer", "{{ @partials/footer }}").unwrap();
    handlebars.register_partial("nav-aside", "{{ @partials/nav-aside }}").unwrap();

    handlebars.register_partial("home-intro", "{{ @partials/home-intro }}").unwrap();
    handlebars.register_partial("latest-articles", "{{ @partials/latest-articles }}").unwrap();
    handlebars.register_partial("projects", "{{ @partials/projects }}").unwrap();
    handlebars.register_partial("newsletter", "{{ @partials/newsletter }}").unwrap();
    handlebars.register_partial("article-series", "{{ @partials/article-series }}").unwrap();
    handlebars.register_partial("categories", "{{ @partials/categories }}").unwrap();

    let handlebars_ref = web::Data::new(handlebars);


    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new()
                .add(("X-Version", "0.2"))
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(handlebars_ref.clone())
            .service(fs::Files::new("/static", "static"))
            .service(home)
            .service(about)
            .service(articles)
            .service(snippets)
            .service(videos)
            .service(article_detail)
            .service(echo)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

