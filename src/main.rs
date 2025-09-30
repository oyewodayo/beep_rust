mod models;
mod handlers;
mod database;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize database connection
    let pool = database::connect().await?;

    // Create application routes - FIXED SYNTAX: use {id} instead of :id
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/topics",
             get(handlers::get_topics)
             .post(handlers::create_topic))
        .route("/topics/{id}", 
            get(handlers::get_topic)
            .put(handlers::update_topic)
            .delete(handlers::delete_topic)
        )
        .route("/topics/slug/{slug}", get(handlers::get_topic_by_slug)) // Add this
        .route("/questions",
            get(handlers::get_questions)
            .post(handlers::create_question))
        .route("/questions/bulk", post(handlers::bulk_create_questions))
        .route("/questions/{id}", 
            get(handlers::get_question)
            .put(handlers::update_question)
            .delete(handlers::delete_question)
        )
        .route("/questions/topic/{topic_id}", get(handlers::get_questions_by_topic))
        .route("/questions/type/{question_type}", get(handlers::get_questions_by_type))
        .route("/questions/search/{query}", get(handlers::search_questions))
        .layer(
            CorsLayer::new().allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
        )
        .with_state(pool);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}