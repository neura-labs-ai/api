use crate::{db::CollectionNames, methods::{generate_api_key, RequestBody}, models::Tokens, AppState};
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId};
use neura_labs_engine::{
    pipelines::translation::generate_translation,
    utils::{concatenate_strings, convert_strings_to_strs},
    Language,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetUserBody {
    pub id: String,
}

#[derive(Deserialize)]
pub struct TranslateBody {
    /// The language to translate from
    source_language: Language,
    /// The language to translate to
    target_language: Language,
    /// The input to translate
    input_context: Vec<String>,
    concat: Option<bool>,
}

#[derive(Serialize, Clone)]
struct TranslateResponse {
    data: Vec<String>,
}

/// Translates a given input from a source language to a target language
///
/// If the `concat` field is set to true, the output will be concatenated into a single string.
/// The result will simply be the first element of the output array.
///
/// # Example Request Body
/// ```json
/// {
///   "data": {
///     "source_language": "English",
///     "target_language": "Spanish",
///     "input_context": [
///     "How are you?",
///     ],
///     "concat": false
///   }
/// }
/// ```
#[post("/api/v1/translate")]
pub async fn translate(body: web::Json<RequestBody<TranslateBody>>) -> impl Responder {
    let result = actix_web::web::block(move || {
        // todo - make sure this works
        let v_langs = vec![body.data.source_language, body.data.target_language];
        for lang in v_langs {
            let validate = validate_language_input(lang);
            if !validate {
                return Err::<TranslateResponse, anyhow::Error>(anyhow::Error::msg(
                    "Invalid language input",
                ));
            }
        }

        let mut res = Vec::new();

        let data = generate_translation(
            body.data.source_language,
            body.data.target_language,
            convert_strings_to_strs(&body.data.input_context),
        )
        .unwrap();

        if body.data.concat.unwrap_or(false) {
            let c = concatenate_strings(&convert_strings_to_strs(&data));
            res.push(c)
        } else {
            res.extend(data);
        };

        Ok::<TranslateResponse, anyhow::Error>(TranslateResponse { data: res })
    })
    .await
    .unwrap()
    .unwrap();

    HttpResponse::Ok().json(result)
}

fn validate_language_input(lang: Language) -> bool {
    match lang {
        Language::English => true,
        Language::Spanish => true,
        Language::French => true,
        _ => false,
    }
}

#[post("/api/v1/token")]
pub async fn create_api_token(
    data: web::Data<AppState>,
    body: web::Json<RequestBody<String>>,
) -> impl Responder {
    let collection = data.db.get_collection::<Tokens>(CollectionNames::Tokens);

    let token = Tokens {
        _id: ObjectId::new(),
        token: generate_api_key(),
        created_at: chrono::Utc::now(),
        updated_at: None,
        tomestoned: false,
        author_id: data.db.convert_to_object_id(body.data.clone()).unwrap(),
    };

    match collection.insert_one(token, None).await {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
