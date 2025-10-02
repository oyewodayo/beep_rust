use axum::{
    extract::{Path, Query, State}, 
    http::StatusCode, 
    Json
};
use serde::Deserialize;
use sqlx::{PgPool}; 
use uuid::Uuid;


use crate::models::{generate_slug, ApiResponse, CreateTopic, Topic, UpdateTopic /* other specific items */}; 

// Topic handlers
pub async fn get_topics(
    State(pool): State<PgPool>,
) -> Result<Json<ApiResponse<Vec<Topic>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let topics = sqlx::query_as::<_, Topic>("SELECT * FROM topics ORDER BY name")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch topics: {}", e))),
            )
        })?;

    Ok(Json(ApiResponse::success(topics)))
}

pub async fn get_topic(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Topic>>, (StatusCode, Json<ApiResponse<()>>)> {
    let topic = sqlx::query_as::<_, Topic>("SELECT * FROM topics WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch topic: {}", e))),
            )
        })?;

    match topic {
        Some(topic) => Ok(Json(ApiResponse::success(topic))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Topic not found".to_string())),
        )),
    }
}

pub async fn delete_topic(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let result = sqlx::query("DELETE FROM topics WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to delete topic: {}", e))),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Topic not found".to_string())),
        ));
    }

    Ok(Json(ApiResponse::success(())))
}

pub async fn create_topic(
    State(pool): State<PgPool>,
    Json(mut payload): Json<CreateTopic>,
) -> Result<Json<ApiResponse<Topic>>, (StatusCode, Json<ApiResponse<()>>)> {
    let slug_is_empty = match &payload.slug {
        Some(s) => s.trim().is_empty(),
        None => true,
    };
    if slug_is_empty {
        payload.slug = Some(generate_slug(&payload.name));
    }

    if let Some(slug) = &mut payload.slug {
        *slug = slug.trim().to_string();
    }

    let topic = sqlx::query_as::<_, Topic>(
        "INSERT INTO topics (name, slug, description) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(payload.name)
    .bind(payload.slug)
    .bind(payload.description)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to create topic: {}", e))),
        )
    })?;

    Ok(Json(ApiResponse::success(topic)))
}

pub async fn update_topic(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(mut payload): Json<UpdateTopic>,
) -> Result<Json<ApiResponse<Topic>>, (StatusCode, Json<ApiResponse<()>>)> {
    if let (Some(name), Some(slug)) = (&payload.name, &payload.slug) {
        if slug.trim().is_empty() {
            payload.slug = Some(generate_slug(name));
        }
    }

    let topic = sqlx::query_as::<_, Topic>(
        "UPDATE topics SET 
            name = COALESCE($1, name), 
            slug = COALESCE($2, slug), 
            description = COALESCE($3, description) 
         WHERE id = $4 RETURNING *"
    )
    .bind(payload.name)
    .bind(payload.slug)
    .bind(payload.description)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to update topic: {}", e))),
        )
    })?;

    match topic {
        Some(topic) => Ok(Json(ApiResponse::success(topic))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Topic not found".to_string())),
        )),
    }
}

pub async fn get_topic_by_slug(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Topic>>, (StatusCode, Json<ApiResponse<()>>)> {
    let topic = sqlx::query_as::<_, Topic>("SELECT * FROM topics WHERE slug = $1")
        .bind(slug)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch topic: {}", e))),
            )
        })?;

    match topic {
        Some(topic) => Ok(Json(ApiResponse::success(topic))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Topic not found".to_string())),
        )),
    }
}


// Helper function
pub async fn get_topic_id_by_slug(pool: &PgPool, slug: &str) -> Result<Uuid, (StatusCode, Json<ApiResponse<()>>)> {
    let topic: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM topics WHERE slug = $1")
        .bind(slug)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Database error: {}", e))),
            )
        })?;

    match topic {
        Some((id,)) => Ok(id),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("Topic with slug '{}' not found", slug))),
        )),
    }
}