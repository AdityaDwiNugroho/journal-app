use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};

use crate::{models::User, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub username: String,
    pub exp: usize,
}

pub fn create_token(user: &User) -> anyhow::Result<String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    let expiration = Utc::now() + Duration::hours(24);
    
    let claims = Claims {
        user_id: user.id,
        username: user.username.clone(),
        exp: expiration.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str) -> anyhow::Result<Claims> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

// Helper function to get current user from cookies
pub async fn get_current_user(
    cookies: &CookieJar,
    state: &AppState,
) -> Option<User> {
    let token = cookies.get("auth_token")?.value();
    let claims = verify_token(token).ok()?;
    
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(claims.user_id)
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten()
}
