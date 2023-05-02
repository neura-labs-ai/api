extern crate anyhow;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use neura_labs_engine::{
    pipelines::translation::generate_translation,
    utils::{concatenate_strings, convert_strings_to_strs},
    Language,
};

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!"); // <- response with app_name
    HttpResponse::Ok().body("Hello world!")
}

#[get("/t")]
async fn translate() -> impl Responder {
    let gen_strings = actix_web::web::block(move || {
        generate_translation(Language::English, Language::Spanish, vec!["How are you?"])
    })
    .await
    .unwrap().unwrap();

    let t = concatenate_strings(&convert_strings_to_strs(&gen_strings));

    HttpResponse::Ok().body(t)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Neura Labs API"),
            }))
            .service(index)
            .service(echo)
            .service(translate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
