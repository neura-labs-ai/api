use actix_web::{post, HttpResponse, web, Responder};
use neura_labs_engine::{
    pipelines::translation::generate_translation,
    utils::{concatenate_strings, convert_strings_to_strs},
    Language,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
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

#[derive(Serialize, Clone)]
struct TranslateResponse {
    data: Vec<String>,
}

#[post("/translate")]
pub async fn translate(body: web::Json<TranslateRequest>) -> impl Responder {
    let result = actix_web::web::block(move || {

        let mut res = Vec::new();

        let data = generate_translation(
            body.source_language,
            body.target_language,
            convert_strings_to_strs(&body.input_context),
        )
        .unwrap();

        if body.concat.unwrap_or(false) {
            let c = concatenate_strings(&convert_strings_to_strs(&data));
            res.push(c)
        } else {
            res.extend(data);
        };

        Ok::<TranslateResponse, anyhow::Error>(
            TranslateResponse {
                data: res,
            }
        )
    })
    .await
    .unwrap().unwrap();

    HttpResponse::Ok().json(result)
}
