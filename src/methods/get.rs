use crate::{AppState, models::{User, Statistics}, db::CollectionNames, methods::RequestBody};
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


/// Returns data for a given user
///
/// # Example Request Body
/// ```json
/// {
///   "data": "user-id"
/// }
/// ```
#[get("/api/v1/user")]
pub async fn get_user(
    data: web::Data<AppState>,
    body: web::Json<RequestBody<String>>,
) -> impl Responder {
    let collection = data.db.get_collection::<User>(CollectionNames::User);

    let id = match data.db.convert_to_object_id(body.data.clone()) {
        Ok(id) => id,
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };

    let filter = doc! {"_id": id};

    match collection.find_one(filter, None).await {
        Ok(result) => match result {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().body("User not found"),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

/// Returns the statistics for a given user
///
/// # Example Request Body
/// ```json
/// {
///   "data": "user-id"
/// }
/// ```
#[get("/api/v1/user/stats")]
pub async fn get_user_stats(
    data: web::Data<AppState>,
    body: web::Json<RequestBody<String>>,
) -> impl Responder {
    let collection = data
        .db
        .get_collection::<Statistics>(CollectionNames::Statistics);

    let id = data.db.convert_to_object_id(body.data.clone()).unwrap();

    let filter = doc! {"_id": id};

    match collection.find_one(filter, None).await {
        Ok(result) => match result {
            Some(data) => HttpResponse::Ok().json(data),
            None => HttpResponse::NotFound().body(format!("No stats found for id: {}.", body.data)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
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