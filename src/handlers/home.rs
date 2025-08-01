use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::extract::CookieJar;
use askama::Template;
use sqlx::Row;
use std::sync::Arc;

use crate::{
    auth::get_current_user,
    models::JournalListItem,
    templates::HomeTemplate,
    AppState,
};

pub async fn home(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> impl IntoResponse {
    let current_user = get_current_user(&cookies, &state).await;
    
    // Get recent published journals
    let journal_rows = sqlx::query(
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
    .await;
    
    let journals = match journal_rows {
        Ok(rows) => {
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
            journals
        }
        Err(_) => {
            // If database query fails, return empty journals
            vec![]
        }
    };
    
    let template = HomeTemplate {
        title: "Journal - Share Your Stories".to_string(),
        journals,
        user: current_user,
    };
    
    Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string()))
}
