use serde_json::json;
use handlebars::Handlebars;
use actix_web::{
    error, Error,
    get,
    web, HttpResponse, Responder, Result,
};

#[get("/news")]
pub async fn news() -> Result<HttpResponse, Error> {
    let reg = Handlebars::new();
    let response = reg
        .render_template("Hello World {{ name }}", &json!({"name":"news"}))
        .map_err(|e| {
            error::ErrorInternalServerError(format!("Template error: {}", e))
        })?;
    Ok(HttpResponse::Ok().body(response))
}

#[get("/")]
async fn home(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({
        "name": "Handlebars",
        // partials
        "header":"partials/layout"
    });
    let body = hb.render("index", &data).unwrap();
    web::Html::new(body)
}
