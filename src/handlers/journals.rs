use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use askama::Template;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::get_current_user,
    models::{Journal, JournalForm, JournalListItem, JournalDetails},
    templates::{JournalListTemplate, JournalNewTemplate, JournalShowTemplate, JournalEditTemplate, MyJournalsTemplate},
    AppState,
};

pub async fn list(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let current_user = get_current_user(&cookies, &state).await;
    
    // Get published journals with user info - using raw SQL for now
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

    let template = JournalListTemplate {
        title: "Journals - Journal".to_string(),
        journals,
        user: current_user,
    };

    Ok(Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string())))
}

pub async fn my_journals(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };
    
    // Get ALL journals for the current user (drafts, private, published)
    let rows = sqlx::query(
        r#"
        SELECT 
            id, title, content, is_published, is_private, created_at, updated_at, published_at
        FROM journals 
        WHERE user_id = $1
        ORDER BY updated_at DESC
        "#
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut journals = Vec::new();
    for row in rows {
        journals.push(Journal {
            id: row.get("id"),
            user_id: user.id,
            title: row.get("title"),
            content: row.get("content"),
            is_published: row.get("is_published"),
            is_private: row.get("is_private"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            published_at: row.get("published_at"),
        });
    }

    let template = MyJournalsTemplate {
        journals,
        user: Some(user),
    };

    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?).into_response())
}

pub async fn new_page(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> impl IntoResponse {
    // Check if user is authenticated
    let current_user = get_current_user(&cookies, &state).await;
    if current_user.is_none() {
        return Redirect::to("/login").into_response();
    }

    let template = JournalNewTemplate {
        title: "New Journal - Journal".to_string(),
        error: None,
        user: current_user,
    };

    Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string())).into_response()
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
    Form(form): Form<JournalForm>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get current user from auth
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    let journal_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    
    // Convert checkbox value to boolean
    let is_private = form.is_private
        .as_ref()
        .map(|s| s == "on" || s == "true")
        .unwrap_or(false);

    let journal = sqlx::query_as::<_, Journal>(
        r#"
        INSERT INTO journals (id, user_id, title, content, is_published, is_private, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(journal_id)
    .bind(user.id)
    .bind(&form.title)
    .bind(&form.content)
    .bind(false) // Always start as draft
    .bind(is_private)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/journals/{}", journal.id)).into_response())
}

pub async fn show(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let current_user = get_current_user(&cookies, &state).await;
    
    // Get journal with user info
    let row = sqlx::query(
        r#"
        SELECT 
            j.id, j.title, j.content, j.is_published, j.is_private, j.published_at,
            u.username, u.display_name
        FROM journals j
        JOIN users u ON j.user_id = u.id
        WHERE j.id = $1
        "#
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let journal = JournalDetails {
        id: row.get("id"),
        title: row.get("title"),
        content: row.get("content"),
        username: row.get("username"),
        display_name: row.get("display_name"),
        published_at: row.get("published_at"),
        is_published: row.get("is_published"),
        is_private: row.get("is_private"),
    };

    let template = JournalShowTemplate {
        title: format!("{} - Journal", journal.title),
        journal,
        user: current_user,
    };

    Ok(Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string())))
}

pub async fn edit_page(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // Get current user from auth
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    // Get the journal
    let journal = sqlx::query_as::<_, Journal>(
        "SELECT * FROM journals WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check if user owns this journal
    if journal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let template = JournalEditTemplate {
        title: format!("Edit: {} - Journal", journal.title),
        journal,
        user: Some(user),
        error: None,
    };

    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?).into_response())
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
    Form(form): Form<JournalForm>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get current user from auth
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    // Check if user owns this journal
    let journal = sqlx::query_as::<_, Journal>(
        "SELECT * FROM journals WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    if journal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Update the journal
    let is_private = form.is_private
        .as_ref()
        .map(|s| s == "on" || s == "true")
        .unwrap_or(false);
        
    sqlx::query(
        "UPDATE journals SET title = $1, content = $2, is_private = $3, updated_at = $4 WHERE id = $5"
    )
    .bind(&form.title)
    .bind(&form.content)
    .bind(is_private)
    .bind(chrono::Utc::now())
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/journals/{}", id)).into_response())
}

pub async fn delete(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // Get current user from auth
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    // Check if user owns this journal
    let journal = sqlx::query_as::<_, Journal>(
        "SELECT * FROM journals WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    if journal.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Delete the journal
    sqlx::query("DELETE FROM journals WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/journals").into_response())
}

pub async fn publish(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if user is authenticated
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    // Check if the journal belongs to the user
    let journal_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM journals WHERE id = $1 AND user_id = $2)"
    )
    .bind(id)
    .bind(user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !journal_exists {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now();
    
    sqlx::query(
        "UPDATE journals SET is_published = true, published_at = $1, updated_at = $2 WHERE id = $3"
    )
    .bind(now)
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/journals/{}", id)).into_response())
}

pub async fn unpublish(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if user is authenticated
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    // Check if the journal belongs to the user
    let journal_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM journals WHERE id = $1 AND user_id = $2)"
    )
    .bind(id)
    .bind(user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !journal_exists {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = chrono::Utc::now();
    
    sqlx::query(
        "UPDATE journals SET is_published = false, published_at = NULL, updated_at = $1 WHERE id = $2"
    )
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/journals/{}", id)).into_response())
}
