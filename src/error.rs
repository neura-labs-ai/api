use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomAPIError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for CustomAPIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CustomAPIError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomAPIError::BadClientData => StatusCode::BAD_REQUEST,
            CustomAPIError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}