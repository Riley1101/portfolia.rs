use actix_web::{get, web, HttpResponse, Result};
use serde::Serialize;
use crate::database::models::DbPool;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/api")] 
async fn api_edit(
    _pool: web::Data<DbPool>,
) -> Result<HttpResponse>{

   let response = Response {
       message: "Resource not found".to_string(),
   };
   Ok(HttpResponse::NotFound().json(response))
}
