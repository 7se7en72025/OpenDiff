use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::db::Database;
use crate::models::{Review, ReviewRequest, ReviewResponse, ApiResponse};
use crate::errors::AppError;
use std::sync::Arc;

pub async fn create_review(
    db: web::Data<Arc<Database>>,
    req: web::Json<ReviewRequest>,
) -> Result<HttpResponse, AppError> {
    let review = Review {
        id: None,
        pr_number: req.pr_number,
        reviewer_id: "reviewer_1".to_string(), // TODO: Get from auth
        reviewer_name: "Team Member".to_string(),
        status: req.status.clone(),
        comment: req.comment.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match db.create_review(review).await {
        Ok(id) => {
            let response = ReviewResponse {
                id,
                pr_number: req.pr_number,
                reviewer_name: "Team Member".to_string(),
                status: req.status.clone(),
                created_at: Utc::now(),
            };
            Ok(HttpResponse::Created().json(ApiResponse::ok(response)))
        }
        Err(_) => Err(AppError::InternalServerError(
            "Failed to create review".to_string(),
        )),
    }
}

pub async fn get_reviews(
    db: web::Data<Arc<Database>>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, AppError> {
    let pr_number = query.get("pr").and_then(|p| p.parse::<i32>().ok());

    match db.get_reviews(pr_number).await {
        Ok(reviews) => Ok(HttpResponse::Ok().json(ApiResponse::ok(reviews))),
        Err(_) => Err(AppError::InternalServerError(
            "Failed to fetch reviews".to_string(),
        )),
    }
}

pub async fn update_review(
    db: web::Data<Arc<Database>>,
    id: web::Path<String>,
    req: web::Json<ReviewRequest>,
) -> Result<HttpResponse, AppError> {
    match db.update_review(&id.into_inner(), &req.status, req.comment.clone()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
            "success": true,
            "message": "Review updated"
        })))),
        Err(_) => Err(AppError::InternalServerError(
            "Failed to update review".to_string(),
        )),
    }
}

pub async fn delete_review(
    db: web::Data<Arc<Database>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    match db.delete_review(&id.into_inner()).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Err(AppError::InternalServerError(
            "Failed to delete review".to_string(),
        )),
    }
}
