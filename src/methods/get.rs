use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello from {app_name}. Visit https://neuralabs.vercel.app for more information about the API!"))
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

#[derive(Serialize, Deserialize)]
struct GlobalStatistics {
    customers: i32,
    api_calls: i32,
    github_stars: i32,
}

/// Returns the global statistics for the application
// todo - fetch from the db
// todo - implement on startup cache that updates every 2 hours.
// todo - this will fetch the latest stats to display globally.
#[get("/api/v1/stats")]
pub async fn get_global_statistics(_data: web::Data<AppState>) -> impl Responder {
    let customers = 500;
    let api_calls = 1000;
    let github_stars = 100;

    let stats = GlobalStatistics {
        customers,
        api_calls,
        github_stars,
    };

    HttpResponse::Ok().json(stats)
}
