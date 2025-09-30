//models.rs
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Type;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use regex::Regex;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Topic {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTopic {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkCreateQuestions {
    pub topic_slug: String,  
    pub questions: Vec<CreateQuestion>,
}

pub struct BulkCreateResponse {
    pub created: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTopic {
    pub name: Option<String>,
    pub description: Option<String>,
    pub(crate) slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[sqlx(type_name = "question_type", rename_all = "lowercase")]
pub enum QuestionType {
    Single,
    Multiple,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Question {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub question_number: i32,
    pub question: String,
    pub options: serde_json::Value,
    pub correct_answer: serde_json::Value,
    pub explanation: String,
    pub question_type: QuestionType,
    pub difficulty: Difficulty,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub topic_id: Uuid,
    pub question_number: i32,
    pub question: String,
    pub options: serde_json::Value,
    pub correct_answer: serde_json::Value,
    pub explanation: String,
    pub question_type: QuestionType,
    pub difficulty: Option<Difficulty>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestion {
    pub topic_id: Option<Uuid>,
    pub question_number: Option<i32>,
    pub question: Option<String>,
    pub options: Option<serde_json::Value>,
    pub correct_answer: Option<serde_json::Value>,
    pub explanation: Option<String>,
    pub question_type: Option<QuestionType>,
    pub difficulty: Option<Difficulty>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizOption {
    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
    pub e: Option<String>,
    pub f: Option<String>,
}

// Response types
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }
}

impl ApiResponse<()> {
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: (),
            message: Some(message),
        }
    }
}


pub fn generate_slug(name: &str) -> String {
    // Convert to lowercase
    let slug = name.to_lowercase();
    
    // Replace spaces and special characters with hyphens
    let slug = slug.replace(" ", "-");
    
    // Remove any remaining special characters except hyphens and alphanumeric
    let re = Regex::new(r"[^a-z0-9-]").unwrap();
    let slug = re.replace_all(&slug, "").to_string();
    
    // Remove consecutive hyphens
    let re = Regex::new(r"-+").unwrap();
    let slug = re.replace_all(&slug, "-").to_string();
    
    // Trim hyphens from start and end
    let slug = slug.trim_matches('-').to_string();
    
    // If slug is empty, generate a hash-based one
    if slug.is_empty() {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        format!("topic-{}", hasher.finish())
    } else {
        slug
    }
}