
mod database;
mod handlers;
mod models;

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

    // Define all app routes
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route(
            "/topics",
            get(handlers::topic::get_topics).post(handlers::topic::create_topic),
        )
        .route(
            "/topics/{id}",
            get(handlers::topic::get_topic)
                .put(handlers::topic::update_topic)
                .delete(handlers::topic::delete_topic),
        )
        .route("/topics/slug/{slug}", get(handlers::topic::get_topic_by_slug))
        .route(
            "/questions",
            get(handlers::question::get_questions).post(handlers::question::create_question),
        )
        .route("/questions/bulk", post(handlers::question::bulk_create_questions))
        .route(
            "/questions/{id}",
            get(handlers::question::get_question)
                .put(handlers::question::update_question)
                .delete(handlers::question::delete_question),
        )
        .route(
            "/questions/topic/{topic_id}",
            get(handlers::question::get_questions_by_topic),
        )
        .route(
            "/questions/type/{question_type}",
            get(handlers::question::get_questions_by_type),
        )
        .route("/questions/search/{query}", get(handlers::question::search_questions))
        .with_state(pool);

    // Wrap with /api prefix
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
