use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BucketItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub status: String, // "active", "completed"
    pub created_at: DateTime<Utc>,
    pub last_touched_at: DateTime<Utc>,
    pub future_message: String,
    pub photos: Vec<String>,
}
