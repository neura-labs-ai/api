use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello from {app_name}!"))
}

#[derive(Serialize, Deserialize)]
struct HealthReport {
    server: String,
    database: String,
}

#[get("/health")]
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let ping_result = data.db.ping().await.unwrap();

    let health_report = HealthReport {
        server: "OK".to_string(),
        database: ping_result.get("ok").unwrap().to_string(),
    };

    if health_report.database == "1" {
        HttpResponse::Ok().json(health_report)
    } else {
        HttpResponse::InternalServerError().json(HealthReport {
            server: "OK".to_string(),
            database: "0".to_string(),
        })
    }
}
