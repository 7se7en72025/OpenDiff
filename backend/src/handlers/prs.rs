use actix_web::{web, HttpResponse};
use chrono::Utc;
use crate::db::Database;
use crate::models::{PullRequest, ApiResponse};
use crate::errors::AppError;
use std::sync::Arc;

pub async fn get_prs(
    db: web::Data<Arc<Database>>,
) -> Result<HttpResponse, AppError> {
    match db.get_prs().await {
        Ok(prs) => {
            // Also get team stats
            match db.get_team_stats().await {
                Ok(stats) => {
                    Ok(HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
                        "pull_requests": prs,
                        "stats": stats
                    }))))
                }
                Err(_) => Ok(HttpResponse::Ok().json(ApiResponse::ok(prs)))
            }
        }
        Err(_) => Err(AppError::InternalServerError(
            "Failed to fetch PRs".to_string(),
        )),
    }
}

pub async fn sync_prs(
    db: web::Data<Arc<Database>>,
) -> Result<HttpResponse, AppError> {
    // In a real implementation, this would fetch from GitHub API
    // and update the database with latest PR data
    
    match db.get_team_stats().await {
        Ok(stats) => {
            Ok(HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
                "success": true,
                "message": "PRs synced successfully",
                "stats": stats
            }))))
        }
        Err(_) => Err(AppError::InternalServerError(
            "Failed to sync PRs".to_string(),
        )),
    }
}
