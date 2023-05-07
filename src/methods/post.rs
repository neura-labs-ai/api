use crate::{
    db::CollectionNames,
    methods::{generate_api_key, RequestBody},
    models::{Credits, Payment, Tokens},
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
pub struct GetUserBody {
    pub id: String,
}

#[derive(Deserialize, Clone)]
pub struct TranslateBody {
    /// The language to translate from
    source_language: Language,
    /// The language to translate to
    target_language: Language,
    /// The input to translate
    input_context: Vec<String>,
    concat: Option<bool>,
    id: String,
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
///     "World Hello",
///     ],
///     "concat": false
///   }
/// }
/// ```
#[post("/api/v1/translate")]
pub async fn translate(
    data: web::Data<AppState>,
    body: web::Json<RequestBody<TranslateBody>>,
) -> impl Responder {
    let body_data = body.data.clone();

    let id = data.db.convert_to_object_id(body_data.id).unwrap();
    let can_run = data.db.process_credit_usage(id).await.unwrap();

    if !can_run {
        return HttpResponse::BadRequest().body("Insufficient credit amount!");
    }

    let result = actix_web::web::block(move || {
        // todo - make sure this works
        let v_langs = vec![body_data.source_language, body_data.target_language];
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
            body_data.source_language,
            body_data.target_language,
            convert_strings_to_strs(&body_data.input_context),
        )
        .unwrap();

        if body_data.concat.unwrap_or(false) {
            let c = concatenate_strings(&convert_strings_to_strs(&data));
            res.push(c)
        } else {
            res.extend(data);
        };

        Ok::<TranslateResponse, anyhow::Error>(TranslateResponse { data: res })
    })
    .await
    .unwrap();

    match result {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{}", e));
        }
    };
}

fn validate_language_input(lang: Language) -> bool {
    match lang {
        Language::English => true,
        Language::Spanish => true,
        Language::French => true,
        _ => false,
    }
}

// todo - block users from accessing this endpoint
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
        userId: data.db.convert_to_object_id(body.data.clone()).unwrap(),
    };

    match collection.insert_one(token, None).await {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

#[derive(Deserialize, Clone)]
pub struct CreatePaymentBody {
    pub id: String,
    pub amount: i32,
    // pub payment_id: String,
}

// todo - block users from accessing this endpoint
#[post("/api/v1/payment")]
pub async fn create_user_payment(
    data: web::Data<AppState>,
    body: web::Json<RequestBody<CreatePaymentBody>>,
) -> impl Responder {
    let body_data = body.data.clone();
    let uid = data.db.convert_to_object_id(body.data.id.clone()).unwrap();
    let payment_collection = data.db.get_collection::<Payment>(CollectionNames::Payment);
    let credits_collection = data.db.get_collection::<Credits>(CollectionNames::Credits);

    let now = chrono::Utc::now();

    let payment = Payment {
        _id: ObjectId::new(),
        userId: uid,
        active: true,
        subscription_id: body_data.id, // todo - use a generated id from the subscription provider
        subscription_date: now,
        subscription_end_date: now + chrono::Duration::days(30),
        subscription_cancelled: false,
        subscription_cancelled_date: None,
        subscription_cancelled_reason: None,
        credits_purchased: body_data.amount,
    };

    payment_collection.insert_one(payment, None).await.unwrap();

    // check if the user has an existing credit record
    let filter = doc! {"userId": uid};

    let credits = credits_collection.find_one(filter, None).await.unwrap();

    if credits.is_some() {
        let credits = credits.unwrap();
        let new_credits = credits.current_amount.unwrap_or(0) + body_data.amount;
        let filter = doc! {"userId": uid};
        let update = doc! {"$set": {"credits": new_credits}};
        credits_collection
            .update_one(filter, update, None)
            .await
            .unwrap();
    } else {
        let credits = Credits {
            _id: ObjectId::new(),
            userId: uid,
            current_amount: Some(body_data.amount),
            used_amount: Some(0),
        };

        credits_collection.insert_one(credits, None).await.unwrap();
    }

    HttpResponse::Ok().body("ok")
}
