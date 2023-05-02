extern crate anyhow;
use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};

mod methods;
use methods::{
    get::{health_check, index, translate},
};

// This struct represents state

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Neura Labs API"),
            }))
            .service(index)
            .service(health_check)
            .service(translate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
