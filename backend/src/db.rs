use mongodb::{Database as MongoDb, bson::doc, Collection};
use crate::models::{User, Review, PullRequest, TeamStats};
use chrono::Utc;
use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    db: Arc<MongoDb>,
}

impl Database {
    pub fn new(db: MongoDb) -> Self {
        Self { db: Arc::new(db) }
    }

    pub async fn health_check(&self) -> Result<()> {
        self.db.run_command(doc! { "ping": 1 }, None).await?;
        Ok(())
    }

    // User operations
    pub async fn find_user(&self, github_id: &str) -> Result<Option<User>> {
        let collection: Collection<User> = self.db.collection("users");
        Ok(collection.find_one(doc! { "github_id": github_id }, None).await?)
    }

    pub async fn create_user(&self, user: User) -> Result<String> {
        let collection: Collection<User> = self.db.collection("users");
        let result = collection.insert_one(user, None).await?;
        Ok(result.inserted_id.to_string())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let collection: Collection<User> = self.db.collection("users");
        Ok(collection.find(doc! {}, None).await?.try_collect::<Vec<_>>().await?)
    }

    // Review operations
    pub async fn create_review(&self, review: Review) -> Result<String> {
        let collection: Collection<Review> = self.db.collection("reviews");
        let result = collection.insert_one(review, None).await?;
        Ok(result.inserted_id.to_string())
    }

    pub async fn get_reviews(&self, pr_number: Option<i32>) -> Result<Vec<Review>> {
        let collection: Collection<Review> = self.db.collection("reviews");
        
        let filter = if let Some(pr_num) = pr_number {
            doc! { "pr_number": pr_num }
        } else {
            doc! {}
        };
        
        let mut cursor = collection.find(filter, None).await?;
        let mut reviews = Vec::new();
        
        while cursor.advance().await? {
            reviews.push(cursor.deserialize_current()?);
        }
        
        Ok(reviews)
    }

    pub async fn update_review(&self, id: &str, status: &str, comment: Option<String>) -> Result<()> {
        let collection: Collection<Review> = self.db.collection("reviews");
        let oid = mongodb::bson::oid::ObjectId::parse_str(id)?;
        
        collection.update_one(
            doc! { "_id": oid },
            doc! {
                "$set": {
                    "status": status,
                    "comment": comment,
                    "updated_at": Utc::now()
                }
            },
            None,
        ).await?;
        
        Ok(())
    }

    pub async fn delete_review(&self, id: &str) -> Result<()> {
        let collection: Collection<Review> = self.db.collection("reviews");
        let oid = mongodb::bson::oid::ObjectId::parse_str(id)?;
        collection.delete_one(doc! { "_id": oid }, None).await?;
        Ok(())
    }

    // PR operations
    pub async fn upsert_pr(&self, pr: PullRequest) -> Result<()> {
        let collection: Collection<PullRequest> = self.db.collection("pull_requests");
        
        collection.update_one(
            doc! { "number": pr.number },
            doc! {
                "$set": {
                    "title": &pr.title,
                    "author": &pr.author,
                    "status": &pr.status,
                    "changed_files": pr.changed_files,
                    "additions": pr.additions,
                    "deletions": pr.deletions,
                    "url": &pr.url,
                    "synced_at": Utc::now(),
                }
            },
            Some(mongodb::options::UpdateOptions::builder().upsert(true).build()),
        ).await?;
        
        Ok(())
    }

    pub async fn get_prs(&self) -> Result<Vec<PullRequest>> {
        let collection: Collection<PullRequest> = self.db.collection("pull_requests");
        Ok(collection.find(doc! {}, None).await?.try_collect::<Vec<_>>().await?)
    }

    pub async fn get_team_stats(&self) -> Result<TeamStats> {
        let reviews: Vec<Review> = self.get_reviews(None).await?;
        let users: Vec<User> = self.get_all_users().await?;
        
        let approved_count = reviews.iter().filter(|r| r.status == "APPROVED").count() as i32;
        let pending_count = reviews.iter().filter(|r| r.status == "PENDING").count() as i32;
        
        Ok(TeamStats {
            total_reviews: reviews.len() as i32,
            approved_count,
            pending_count,
            active_users: users.len() as i32,
            last_sync: Utc::now(),
        })
    }
}
