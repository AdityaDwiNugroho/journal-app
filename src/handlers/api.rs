use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use sqlx::Row;
use std::sync::Arc;

use crate::{AppState, models::JournalListItem};

pub async fn journals_list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let rows = sqlx::query(
        r#"
        SELECT 
            j.id, j.title, j.content, j.published_at,
            u.username, u.display_name
        FROM journals j
        JOIN users u ON j.user_id = u.id
        WHERE j.is_published = true AND j.is_private = false
        ORDER BY j.published_at DESC
        LIMIT 20
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut journals = Vec::new();
    for row in rows {
        journals.push(JournalListItem {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            username: row.get("username"),
            display_name: row.get("display_name"),
            published_at: row.get("published_at"),
        });
    }

    Ok(Json(json!({
        "journals": journals
    })))
}

pub async fn feed(State(_state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement personalized feed based on following
    Ok(Json(json!({
        "message": "Feed endpoint - coming soon!"
    })))
}
