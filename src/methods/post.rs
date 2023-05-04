use crate::{
    db::CollectionNames,
    methods::generate_api_key,
    models::{Statistics, Tokens, User},
    AppState,
};
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId};
use neura_labs_engine::{
    pipelines::translation::generate_translation,
    utils::{concatenate_strings, convert_strings_to_strs},
    Language,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestBody<T> {
    data: T,
}

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

/// Returns data for a given user
///
/// # Example Request Body
/// ```json
/// {
///   "data": "user-id"
/// }
/// ```
#[post("/api/v1/user")]
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
#[post("/api/v1/user/stats")]
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
