use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "internal server error")]
    InternalError,

    #[display(fmt = "bad request error")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },

    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "unauthorized")]
    Unauthorized,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            // 500's
            ServerError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ServerError::BadClientData => StatusCode::BAD_REQUEST,
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            // 400's
            ServerError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServerError::NotFound => StatusCode::NOT_FOUND,
            ServerError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
