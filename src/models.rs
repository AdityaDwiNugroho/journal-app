use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Journal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Follow {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub following_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct JournalForm {
    pub title: String,
    pub content: String,
    pub is_private: Option<String>, // Changed to String to handle "on" from checkbox
}

#[derive(Debug, Clone, Serialize)]
pub struct JournalWithUser {
    pub journal: Journal,
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub user: User,
    pub journals_count: i64,
    pub followers_count: i64,
    pub following_count: i64,
    pub is_following: bool,
    pub journals: Vec<JournalListItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JournalListItem {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub username: String,
    pub display_name: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JournalDetails {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub username: String,
    pub display_name: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub is_published: bool,
    pub is_private: bool,
}
