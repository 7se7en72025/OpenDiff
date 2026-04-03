use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_cors::Cors;
use mongodb::{Client, bson::doc};
use dotenv::dotenv;
use std::sync::Arc;
use std::env;

mod handlers;
mod models;
mod db;
mod ws;
mod errors;

use handlers::{auth, reviews, prs};
use db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // MongoDB Connection
    let mongo_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("❌ Failed to connect to MongoDB");

    let db = client.database("opendiff");
    let database = Arc::new(Database::new(db));

    // Test connection
    match database.health_check().await {
        Ok(_) => println!("✅ MongoDB connected"),
        Err(e) => println!("❌ MongoDB error: {}", e),
    }

    let db_clone = database.clone();

    println!("🚀 Starting OpenDiff Backend...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db_clone.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            
            // Routes
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(|| async {
                HttpResponse::Ok().content_type("text/html").body(include_str!("../../frontend/public/index.html"))
            }))
            .service(
                web::scope("/api/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/register", web::post().to(auth::register))
                    .route("/me", web::get().to(auth::get_user))
            )
            .service(
                web::scope("/api/reviews")
                    .route("", web::get().to(reviews::get_reviews))
                    .route("", web::post().to(reviews::create_review))
                    .route("/{id}", web::put().to(reviews::update_review))
                    .route("/{id}", web::delete().to(reviews::delete_review))
            )
            .service(
                web::scope("/api/prs")
                    .route("", web::get().to(prs::get_prs))
                    .route("/sync", web::post().to(prs::sync_prs))
            )
            .route("/ws", web::get().to(ws::ws_handler))
    })
    .bind("0.0.0.0:5000")
    .expect("Failed to bind to port 5000")
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .json(serde_json::json!({
            "status": "ok",
            "message": "OpenDiff Backend Running 🚀",
            "timestamp": chrono::Utc::now()
        }))
}
