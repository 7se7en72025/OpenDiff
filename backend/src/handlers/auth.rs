use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::db::Database;
use crate::models::{User, LoginRequest, ApiResponse};
use crate::errors::AppError;
use std::sync::Arc;

pub async fn login(
    db: web::Data<Arc<Database>>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user exists
    match db.find_user(&req.github_id).await {
        Ok(Some(user)) => {
            // Return existing user
            Ok(HttpResponse::Ok().json(ApiResponse::ok(user)))
        }
        Ok(None) => {
            // Create new user
            let new_user = User {
                id: None,
                github_id: req.github_id.clone(),
                name: req.name.clone(),
                email: None,
                avatar: None,
                token: Some(uuid::Uuid::new_v4().to_string()),
                created_at: Utc::now(),
            };

            match db.create_user(new_user.clone()).await {
                Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::ok(new_user))),
                Err(_) => Err(AppError::InternalServerError(
                    "Failed to create user".to_string(),
                )),
            }
        }
        Err(_) => Err(AppError::InternalServerError(
            "Database error".to_string(),
        )),
    }
}

pub async fn register(
    db: web::Data<Arc<Database>>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if user already exists
    if let Ok(Some(_)) = db.find_user(&req.github_id).await {
        return Err(AppError::BadRequest("User already exists".to_string()));
    }

    let new_user = User {
        id: None,
        github_id: req.github_id.clone(),
        name: req.name.clone(),
        email: None,
        avatar: None,
        token: Some(uuid::Uuid::new_v4().to_string()),
        created_at: Utc::now(),
    };

    match db.create_user(new_user.clone()).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::ok(new_user))),
        Err(_) => Err(AppError::InternalServerError(
            "Failed to create user".to_string(),
        )),
    }
}

pub async fn get_user(
    db: web::Data<Arc<Database>>,
) -> Result<HttpResponse, AppError> {
    match db.get_all_users().await {
        Ok(users) => Ok(HttpResponse::Ok().json(ApiResponse::ok(users))),
        Err(_) => Err(AppError::InternalServerError(
            "Failed to fetch users".to_string(),
        )),
    }
}
