use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;
use serde::Serialize;
use sqlx::error::DatabaseError;
use thiserror::Error;
use tracing::debug;
use tracing_subscriber::field::display;
use validator::ValidationErrors;
pub mod authentication;

#[derive(Debug, Display, Error)]
enum ApiError {
    #[display("Validation error")]
    ValidationError(#[from] ValidationErrors),
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Serialize,Display,Debug)]
enum ErrorToRet<'a>{
    #[display("Username already registered")]
    AlreadyRegistered {message: &'a str},
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        debug!("{}", &self);
        let msg = match &self {
            Self::ValidationError(e) => serde_json::to_string(&e).expect("Unable to jsonify error"),
            ApiError::DatabaseError(error) => {
                match error {
                    sqlx::Error::Database(_) => {
                        serde_json::to_string(&ErrorToRet::AlreadyRegistered { message: "Username or email already taken" }).unwrap()
                    },
                    _ => {
                        tracing::error!("Unhandled database error {:?}",error);
                        String::from("Unhandled INTERNAL_SERVER_ERROR")
                    }
                }
            }
        };
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(msg)
    }
}
