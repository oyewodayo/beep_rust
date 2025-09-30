//handler.rs
use axum::{
    extract::{Path, Query, State}, http::StatusCode, Json
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;

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

// Question handlers
#[derive(Debug, Deserialize)]
pub struct QuestionQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_questions(
    State(pool): State<PgPool>,
    Query(query): Query<QuestionQuery>,
) -> Result<Json<ApiResponse<Vec<Question>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).max(1).min(100);
    let offset = (page - 1) * limit;

    let questions = sqlx::query_as::<_, Question>(
        "SELECT q.* FROM questions q 
         JOIN topics t ON q.topic_id = t.id 
         ORDER BY t.name, q.question_number 
         LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to fetch questions: {}", e))),
        )
    })?;

    Ok(Json(ApiResponse::success(questions)))
}

pub async fn get_question(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Question>>, (StatusCode, Json<ApiResponse<()>>)> {
    let question = sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch question: {}", e))),
            )
        })?;

    match question {
        Some(question) => Ok(Json(ApiResponse::success(question))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Question not found".to_string())),
        )),
    }
}

pub async fn create_question(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuestion>,
) -> Result<Json<ApiResponse<Question>>, (StatusCode, Json<ApiResponse<()>>)> {
    let difficulty = payload.difficulty.unwrap_or(Difficulty::Medium);
    
    let question = sqlx::query_as::<_, Question>(
        "INSERT INTO questions (
            topic_id, question_number, question, options, correct_answer, 
            explanation, question_type, difficulty, tags
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(payload.topic_id)
    .bind(payload.question_number)
    .bind(payload.question)
    .bind(payload.options)
    .bind(payload.correct_answer)
    .bind(payload.explanation)
    .bind(payload.question_type)
    .bind(difficulty)
    .bind(payload.tags)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to create question: {}", e))),
        )
    })?;

    Ok(Json(ApiResponse::success(question)))
}

pub async fn update_question(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateQuestion>,
) -> Result<Json<ApiResponse<Question>>, (StatusCode, Json<ApiResponse<()>>)> {
    let question = sqlx::query_as::<_, Question>(
        "UPDATE questions SET 
            topic_id = COALESCE($1, topic_id),
            question_number = COALESCE($2, question_number),
            question = COALESCE($3, question),
            options = COALESCE($4, options),
            correct_answer = COALESCE($5, correct_answer),
            explanation = COALESCE($6, explanation),
            question_type = COALESCE($7, question_type),
            difficulty = COALESCE($8, difficulty),
            tags = COALESCE($9, tags)
         WHERE id = $10 RETURNING *"
    )
    .bind(payload.topic_id)
    .bind(payload.question_number)
    .bind(payload.question)
    .bind(payload.options)
    .bind(payload.correct_answer)
    .bind(payload.explanation)
    .bind(payload.question_type)
    .bind(payload.difficulty)
    .bind(payload.tags)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to update question: {}", e))),
        )
    })?;

    match question {
        Some(question) => Ok(Json(ApiResponse::success(question))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Question not found".to_string())),
        )),
    }
}

pub async fn delete_question(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let result = sqlx::query("DELETE FROM questions WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to delete question: {}", e))),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Question not found".to_string())),
        ));
    }

    Ok(Json(ApiResponse::success(())))
}

// Specialized question handlers
pub async fn get_questions_by_topic(
    State(pool): State<PgPool>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<Question>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let questions = sqlx::query_as::<_, Question>(
        "SELECT * FROM questions WHERE topic_id = $1 ORDER BY question_number"
    )
    .bind(topic_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to fetch questions: {}", e))),
        )
    })?;

    Ok(Json(ApiResponse::success(questions)))
}

pub async fn get_questions_by_type(
    State(pool): State<PgPool>,
    Path(question_type): Path<String>,
) -> Result<Json<ApiResponse<Vec<Question>>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Convert string to QuestionType
    let q_type = match question_type.to_lowercase().as_str() {
        "single" => QuestionType::Single,
        "multiple" => QuestionType::Multiple,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Invalid question type. Use 'single' or 'multiple'".to_string())),
            ));
        }
    };
    
    let questions = sqlx::query_as::<_, Question>(
        "SELECT q.* FROM questions q 
         JOIN topics t ON q.topic_id = t.id 
         WHERE q.question_type = $1 
         ORDER BY t.name, q.question_number"
    )
    .bind(q_type)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to fetch questions: {}", e))),
        )
    })?;

    Ok(Json(ApiResponse::success(questions)))
}

// Add a helper function to get topic_id from slug
async fn get_topic_id_by_slug(pool: &PgPool, slug: &str) -> Result<Uuid, (StatusCode, Json<ApiResponse<()>>)> {
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

// Update create_topic to handle slug
pub async fn create_topic(
    State(pool): State<PgPool>,
    Json(mut payload): Json<CreateTopic>, // Make payload mutable
) -> Result<Json<ApiResponse<Topic>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Auto-generate slug if empty
    let slug_is_empty = match &payload.slug {
        Some(s) => s.trim().is_empty(),
        None => true,
    };
    if slug_is_empty {
        payload.slug = Some(crate::models::generate_slug(&payload.name));
    }

    // Clean up the slug (remove extra spaces, etc.)
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

// Update update_topic to handle slug
pub async fn update_topic(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(mut payload): Json<UpdateTopic>, // Make payload mutable
) -> Result<Json<ApiResponse<Topic>>, (StatusCode, Json<ApiResponse<()>>)> {
    // If name is being updated and slug is provided but empty, generate from new name
    if let (Some(name), Some(slug)) = (&payload.name, &payload.slug) {
        if slug.trim().is_empty() {
            payload.slug = Some(crate::models::generate_slug(name));
        }
    }
    // If only name is being updated but slug isn't provided, we don't change the slug
    // This prevents breaking existing links when only the name changes

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
// Add endpoint to get topic by slug
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


// Update bulk create to use topic_slug instead of topic_id
pub async fn bulk_create_questions(
    State(pool): State<PgPool>,
    Json(payload): Json<BulkCreateQuestions>,
) -> Result<Json<ApiResponse<BulkCreateResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Get topic_id from slug
    let topic_id = get_topic_id_by_slug(&pool, &payload.topic_slug).await?;

    let mut created = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    let mut transaction = pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to start transaction: {}", e))),
        )
    })?;

    for (index, question_data) in payload.questions.iter().enumerate() {
        let result = sqlx::query_as::<_, Question>(
            "INSERT INTO questions (
                topic_id, question_number, question, options, correct_answer, 
                explanation, question_type, difficulty, tags
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
        )
        .bind(topic_id)
        .bind(question_data.question_number)
        .bind(&question_data.question)
        .bind(&question_data.options)
        .bind(&question_data.correct_answer)
        .bind(&question_data.explanation)
        .bind(question_data.question_type.clone())
        .bind(question_data.difficulty.clone().unwrap_or(Difficulty::Medium))
        .bind(question_data.tags.clone())
        .fetch_optional(&mut *transaction)
        .await;

        match result {
            Ok(Some(_)) => created += 1,
            Ok(None) => {
                failed += 1;
                errors.push(format!("Question {}: No data returned", index + 1));
            }
            Err(e) => {
                failed += 1;
                errors.push(format!("Question {}: {}", index + 1, e));
            }
        }
    }

    if failed == 0 {
        transaction.commit().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to commit transaction: {}", e))),
            )
        })?;
    } else {
        transaction.rollback().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to rollback transaction: {}", e))),
            )
        })?;
    }

    let response = BulkCreateResponse {
        created,
        failed,
        errors,
    };

    Ok(Json(ApiResponse::success(response)))
}


pub async fn search_questions(
    State(pool): State<PgPool>,
    Path(query): Path<String>,
) -> Result<Json<ApiResponse<Vec<Question>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let search_pattern = format!("%{}%", query);
    
    let questions = sqlx::query_as::<_, Question>(
        "SELECT q.* FROM questions q 
         JOIN topics t ON q.topic_id = t.id 
         WHERE q.question ILIKE $1 OR q.explanation ILIKE $1 OR t.name ILIKE $1
         ORDER BY t.name, q.question_number"
    )
    .bind(search_pattern)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Failed to search questions: {}", e))),
        )
    })?;

    Ok(Json(ApiResponse::success(questions)))
}