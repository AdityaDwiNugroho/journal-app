use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::CookieJar;
use askama::Template;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::get_current_user,
    models::{User, UserProfile},
    templates::{ProfileTemplate, ProfileEditTemplate},
    AppState,
};

#[derive(Deserialize)]
pub struct ProfileUpdateForm {
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

pub async fn profile(
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let current_user = get_current_user(&cookies, &state).await;
    
    // Get user profile
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&username)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Get user stats
    let journals_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM journals WHERE user_id = $1 AND is_published = true"
    )
    .bind(user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let followers_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM follows WHERE following_id = $1"
    )
    .bind(user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let following_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM follows WHERE follower_id = $1"
    )
    .bind(user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if current user is following this user
    let is_following = if let Some(ref current_user) = current_user {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM follows WHERE follower_id = $1 AND following_id = $2)"
        )
        .bind(current_user.id)
        .bind(user.id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        false
    };

    let profile = UserProfile {
        user,
        journals_count,
        followers_count,
        following_count,
        is_following,
    };

    let template = ProfileTemplate {
        title: format!("{} - Journal", profile.user.username),
        profile,
        user: current_user,
    };

    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

pub async fn follow(
    Path(user_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if user is authenticated
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    // Can't follow yourself
    if user.id == user_id {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if target user exists
    let target_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !target_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // Insert follow relationship (ignore if already exists)
    let _ = sqlx::query(
        "INSERT INTO follows (follower_id, following_id, created_at) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )
    .bind(user.id)
    .bind(user_id)
    .bind(chrono::Utc::now())
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get username for redirect
    let username: String = sqlx::query_scalar(
        "SELECT username FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/profile/{}", username)).into_response())
}

pub async fn unfollow(
    Path(user_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if user is authenticated
    let current_user = get_current_user(&cookies, &state).await;
    let user = match current_user {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    // Can't unfollow yourself
    if user.id == user_id {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Remove follow relationship
    sqlx::query(
        "DELETE FROM follows WHERE follower_id = $1 AND following_id = $2"
    )
    .bind(user.id)
    .bind(user_id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get username for redirect
    let username: String = sqlx::query_scalar(
        "SELECT username FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/profile/{}", username)).into_response())
}

pub async fn edit_profile_page(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if user is authenticated
    let user = get_current_user(&cookies, &state).await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let template = ProfileEditTemplate {
        title: "Edit Profile - Journal".to_string(),
        user: Some(user.clone()),
        profile_user: user,
    };

    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
    Form(form): Form<ProfileUpdateForm>,
) -> Result<impl IntoResponse, StatusCode> {
    // Check if user is authenticated
    let user = get_current_user(&cookies, &state).await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Update user profile
    sqlx::query(
        "UPDATE users SET display_name = $1, bio = $2, updated_at = $3 WHERE id = $4"
    )
    .bind(&form.display_name)
    .bind(&form.bio)
    .bind(chrono::Utc::now())
    .bind(user.id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/profile/{}", user.username)).into_response())
}
