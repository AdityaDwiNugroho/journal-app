use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use askama::Template;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::{create_token, get_current_user},
    models::{LoginForm, RegisterForm, User},
    templates::{LoginTemplate, RegisterTemplate},
    AppState,
};

pub async fn login_page(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> impl IntoResponse {
    let current_user = get_current_user(&cookies, &state).await;
    
    let template = LoginTemplate {
        title: "Login - Journal".to_string(),
        error: None,
        user: current_user,
    };
    
    Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string()))
}

pub async fn register_page(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
) -> impl IntoResponse {
    let current_user = get_current_user(&cookies, &state).await;
    
    let template = RegisterTemplate {
        title: "Register - Journal".to_string(),
        error: None,
        user: current_user,
    };
    
    Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string()))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
    Form(form): Form<LoginForm>,
) -> Result<(CookieJar, Response), StatusCode> {
    // Find user by username
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&form.username)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match user {
        Some(user) => user,
        None => {
            let template = LoginTemplate {
                title: "Login - Journal".to_string(),
                error: Some("Invalid username or password".to_string()),
                user: None,
            };
            return Ok((cookies, Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string())).into_response()));
        }
    };

    // Verify password
    let password_valid = verify(&form.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !password_valid {
        let template = LoginTemplate {
            title: "Login - Journal".to_string(),
            error: Some("Invalid username or password".to_string()),
            user: None,
        };
        return Ok((cookies, Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string())).into_response()));
    }

    // Create JWT token
    let token = create_token(&user).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Set cookie and redirect
    let cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .build();
    let updated_cookies = cookies.add(cookie);

    Ok((updated_cookies, Redirect::to("/").into_response()))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    cookies: CookieJar,
    Form(form): Form<RegisterForm>,
) -> Result<(CookieJar, Response), StatusCode> {
    // Check if username already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1 OR email = $2"
    )
    .bind(&form.username)
    .bind(&form.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        let template = RegisterTemplate {
            title: "Register - Journal".to_string(),
            error: Some("Username or email already exists".to_string()),
            user: None,
        };
        return Ok((cookies, Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string())).into_response()));
    }

    // Hash password
    let password_hash = hash(&form.password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create new user
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, email, password_hash, display_name, bio, is_private, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(&form.username)
    .bind(&form.email)
    .bind(&password_hash)
    .bind(&form.display_name)
    .bind(None::<String>)
    .bind(false)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create JWT token
    let token = create_token(&user).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Set cookie and redirect
    let cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .build();
    let updated_cookies = cookies.add(cookie);

    Ok((updated_cookies, Redirect::to("/").into_response()))
}

pub async fn logout(cookies: CookieJar) -> (CookieJar, Redirect) {
    let cookie = Cookie::build(("auth_token", ""))
        .path("/")
        .http_only(true)
        .build();
    let updated_cookies = cookies.remove(cookie);
    (updated_cookies, Redirect::to("/"))
}
