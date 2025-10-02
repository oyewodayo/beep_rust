mod provider;
mod certification;
mod topic;
mod question;
mod quiz;

use axum::Router;
use sqlx::PgPool;

pub fn api_routes(pool: PgPool) -> Router {
    Router::new()
        .merge(provider::routes())
        .merge(certification::routes())
        .merge(topic::routes())
        .merge(question::routes())
        .merge(quiz::routes())
        .with_state(pool)
}