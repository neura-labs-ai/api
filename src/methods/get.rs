use actix_web::{get, HttpResponse, web, Responder};
use neura_labs_engine::{
    pipelines::translation::generate_translation,
    utils::{concatenate_strings, convert_strings_to_strs},
    Language,
};

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

#[derive(serde::Deserialize, Clone)]
pub struct TranslateRequest {
    /// The language to translate from
    source_language: Language,
    /// The language to translate to
    target_language: Language,
    /// The input to translate
    input_context: Vec<String>,
    // If true, the output will be concatenated into a single string. This is useful when you have a list of sentences that you want to translate
    // However, don't have all the data in a simple array index.
    concat: Option<bool>,
}

#[get("/translate")]
pub async fn translate(body: web::Json<TranslateRequest>) -> impl Responder {
    let result = actix_web::web::block(move || {

        let data = generate_translation(
            body.source_language,
            body.target_language,
            convert_strings_to_strs(&body.input_context),
        )
        .unwrap();

        let res = if body.concat.unwrap_or(false) {
            concatenate_strings(&convert_strings_to_strs(&data))
        } else {
            serde_json::to_string(&data).unwrap()
        };

        Ok::<_, anyhow::Error>(res)
    })
    .await
    .unwrap().unwrap();

    HttpResponse::Ok().body(result)
}
