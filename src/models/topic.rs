use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;




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
