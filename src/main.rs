mod db;
mod error;
mod methods;
mod middleware;
mod utils;
mod models;

extern crate anyhow;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use methods::{
    get::{health_check, index},
    post::{get_user, get_user_stats, translate},
};

use crate::methods::post::create_api_token;

#[derive(Clone, Debug)]
pub struct AppState {
    app_name: String,
    db: db::MongoDB,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::MongoDB::new(&utils::env().unwrap().mongodb_uri)
        .await
        .expect("Failed to initialize MongoDB");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let app_state = AppState {
            app_name: String::from("Neura Labs API"),
            db: db.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::auth::AuthHandler)
            // get
            .service(index)
            .service(health_check)
            // post
            .service(translate)
            .service(get_user)
            .service(get_user_stats)
            .service(create_api_token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}