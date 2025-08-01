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
