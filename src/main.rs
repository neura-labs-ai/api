mod db;
mod error;
mod methods;
mod models;

extern crate anyhow;

use actix_web::{web, App, HttpServer};
use env_file_reader::read_file;

use methods::{
    get::{health_check, index},
    post::{translate, get_user, get_user_stats},
};

#[derive(Clone, Debug)]
pub struct AppState {
    app_name: String,
    db: db::MongoDB,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_variables = read_file(".env").expect("Failed to read .env file");
    let default_url = String::from("mongodb://localhost:27017");
    let mongo_uri = env_variables.get("MONGODB_URI").unwrap_or(&default_url);

    println!("Connecting to MongoDB at {}", mongo_uri);

    let db = db::MongoDB::new(mongo_uri)
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
            .service(get_user)
            .service(get_user_stats)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
