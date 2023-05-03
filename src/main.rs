extern crate anyhow;

use actix_web::{web, App, HttpServer};

mod db;
mod methods;
mod models;
use methods::{
    get::{health_check, index},
    post::translate,
};

#[derive(Clone, Debug)]
pub struct AppState {
    app_name: String,
    pub db: db::MongoDB,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let db = db::MongoDB::new(uri)
        .await
        .expect("Failed to initialize MongoDB");

    HttpServer::new(move || {
        let app_state = AppState {
            app_name: String::from("Neura Labs API"),
            db: db.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(index)
            .service(health_check)
            .service(translate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
