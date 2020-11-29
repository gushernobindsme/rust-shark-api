use crate::error_handler::CustomError;
use crate::model::{Shark, SharkCommand};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/sharks")]
async fn find() -> Result<HttpResponse, CustomError> {
    let sharks = Shark::find()?;
    Ok(HttpResponse::Ok().json(sharks))
}

#[get("/sharks/{id}")]
async fn get(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let shark = Shark::get(id.into_inner())?;
    Ok(HttpResponse::Ok().json(shark))
}

#[post("/sharks")]
async fn create(command: web::Json<SharkCommand>) -> Result<HttpResponse, CustomError> {
    let shark = Shark::create(command.into_inner())?;
    Ok(HttpResponse::Ok().json(shark))
}

#[put("/sharks/{id}")]
async fn update(
    id: web::Path<i32>,
    command: web::Json<SharkCommand>,
) -> Result<HttpResponse, CustomError> {
    let shark = Shark::update(id.into_inner(), command.into_inner())?;
    Ok(HttpResponse::Ok().json(shark))
}

#[delete("/sharks/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_shark = Shark::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_shark })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(get);
    config.service(create);
    config.service(update);
    config.service(delete);
}
