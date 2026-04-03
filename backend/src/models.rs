use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, Serialize as BsonSerialize};

#[derive(Debug, Clone, Serialize, Deserialize, BsonSerialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub github_id: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    #[serde(skip_serializing)]
    pub token: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, BsonSerialize)]
pub struct Review {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub pr_number: i32,
    pub reviewer_id: String,
    pub reviewer_name: String,
    pub status: String, // "APPROVED", "CHANGES_REQUESTED", "PENDING"
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, BsonSerialize)]
pub struct PullRequest {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub number: i32,
    pub title: String,
    pub author: String,
    pub status: String, // "open", "closed", "merged"
    pub changed_files: i32,
    pub additions: i32,
    pub deletions: i32,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub synced_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamStats {
    pub total_reviews: i32,
    pub approved_count: i32,
    pub pending_count: i32,
    pub active_users: i32,
    pub last_sync: DateTime<Utc>,
}

// Request/Response DTOs

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub github_id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ReviewRequest {
    pub pr_number: i32,
    pub status: String,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: String,
    pub pr_number: i32,
    pub reviewer_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}
