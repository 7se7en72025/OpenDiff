use actix_web::{error::ResponseError, HttpResponse};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({
                    "success": false,
                    "error": msg,
                    "code": 404
                }))
            }
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "success": false,
                    "error": msg,
                    "code": 400
                }))
            }
            AppError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(json!({
                    "success": false,
                    "error": msg,
                    "code": 401
                }))
            }
            AppError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "error": msg,
                    "code": 500
                }))
            }
        }
    }
}
