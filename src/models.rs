use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use sqlx::Type;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use regex::Regex;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// === Domain Models ===
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
pub struct UpdateTopic {
    pub name: Option<String>,
    pub description: Option<String>,
    pub slug: Option<String>,
}

// === Enums with proper serde attributes ===
#[derive(Debug, Serialize, Deserialize, Clone, Type, PartialEq)]
#[sqlx(type_name = "question_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")] 
pub enum QuestionType {
    Single,
    Multiple,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type, PartialEq)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

// === Question Models - Using Json<Vec<String>> for consistency ===
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Question {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub question_number: i32,
    pub question: String,
    pub options: Json<Vec<String>>,      
    pub correct_answer: Json<Vec<String>>,
    pub explanation: String,
    pub question_type: QuestionType,
    pub difficulty: Difficulty,
    pub tags: Option<Json<Vec<String>>>, 
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// For API responses - clean types without Json wrapper
#[derive(Debug, Serialize)]
pub struct QuestionResponse {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub question_number: i32,
    pub question: String,
    #[serde(serialize_with = "serialize_options_as_map")]
    pub options: Vec<String>,
    pub correct_answer: Vec<String>,
    pub explanation: String,
    pub question_type: QuestionType,
    pub difficulty: Difficulty,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


// Custom serializer to convert Vec<String> to {"A": "...", "B": "..."}
fn serialize_options_as_map<S>(
    options: &Vec<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let map: HashMap<String, String> = options
        .iter()
        .enumerate()
        .map(|(i, text)| {
            let label = std::char::from_u32(65 + i as u32)
                .unwrap()
                .to_string();
            (label, text.clone())
        })
        .collect();
    
    map.serialize(serializer)
}

// Fixed From implementation
impl From<Question> for QuestionResponse {
    fn from(q: Question) -> Self {
        Self {
            id: q.id,
            topic_id: q.topic_id,
            question_number: q.question_number,
            question: q.question,
            options: q.options.0,       
            correct_answer: q.correct_answer.0, 
            explanation: q.explanation,     
            question_type: q.question_type,
            difficulty: q.difficulty,
            tags: q.tags.map(|t| t.0),    
            created_at: q.created_at,
            updated_at: q.updated_at,
        }
    }
}

// === Input Models - Vec<String> for easy JSON deserialization ===
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub topic_id: Uuid,
    pub question_number: i32,
    pub question: String,
    pub options: Vec<String>,          
    pub correct_answer: Vec<String>, 
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
    pub options: Option<Vec<String>>,       
    pub correct_answer: Option<Vec<String>>, 
    pub explanation: Option<String>,
    pub question_type: Option<QuestionType>,
    pub difficulty: Option<Difficulty>,
    pub tags: Option<Vec<String>>,
}

// === Bulk Operations ===

#[derive(Debug, Deserialize)]
pub struct BulkCreateQuestions {
    pub topic_slug: String,  
    pub questions: Vec<BulkQuestionData>,
}

#[derive(Debug, Deserialize)]
pub struct BulkQuestionData {
    pub question_number: i32,
    pub question: String,
    pub options: Vec<String>,          
    pub correct_answer: Vec<String>,   
    pub explanation: String,           
    pub question_type: QuestionType,
    pub difficulty: Option<Difficulty>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct BulkCreateResponse {
    pub created: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

// === Response Types ===
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

// === Utility Functions ===
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


impl BulkQuestionData {
    /// Convert to CreateQuestion for reusing existing handler logic
    pub fn to_create_question(&self, topic_id: Uuid) -> CreateQuestion {
        CreateQuestion {
            topic_id,
            question_number: self.question_number,
            question: self.question.clone(),
            options: self.options.clone(),
            correct_answer: self.correct_answer.clone(),
            explanation: self.explanation.clone(),
            question_type: self.question_type.clone(),
            difficulty: self.difficulty.clone(),
            tags: self.tags.clone(),
        }
    }
}