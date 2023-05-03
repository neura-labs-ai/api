use actix_web::{get, HttpResponse, web, Responder};
use crate::AppState;

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello from {app_name}!"))
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}