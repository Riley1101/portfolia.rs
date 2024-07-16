mod route;
use actix_web::middleware::Logger;
use actix_web::{ App, HttpServer, web, middleware};
use env_logger::Env;
use handlebars::{DirectorySourceOptions, Handlebars};
use route::echo;
use route::home::{ home , news};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

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
            .service(news)
            .service(echo)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

