extern crate anyhow;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use neura_labs_engine::{
    pipelines::translation::generate_translation,
    utils::{concatenate_strings, convert_strings_to_strs},
    Language,
};

// let gen_strings =
//     generate_translation(Language::English, Language::Spanish, vec!["How are you?"]).unwrap();

// let t = convert_strings_to_strs(&gen_strings);

// println!("{}", concatenate_strings(&t));

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
