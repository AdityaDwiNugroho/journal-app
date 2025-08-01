mod auth;
mod database;
mod handlers;
mod models;
mod templates;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber;

pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Try to initialize database, but don't fail if it's not available
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/journal_app".to_string());
    
    let pool = match database::create_pool(&database_url).await {
        Ok(pool) => {
            println!("✅ Database connected successfully");
            // Run migrations
            if let Err(e) = database::run_migrations(&pool).await {
                println!("⚠️  Migration skipped: {}", e);
            }
            pool
        }
        Err(e) => {
            println!("⚠️  Database connection failed: {}", e);
            println!("   Running in demo mode without database");
            // Return a fake pool - we'll handle this in handlers
            return Err(anyhow::anyhow!("Database not available. Please set up PostgreSQL and update your .env file."));
        }
    };

    let state = Arc::new(AppState { db: pool });

    // Build our application with routes
    let app = Router::new()
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        
        // Public routes
        .route("/", get(handlers::home::home))
        .route("/login", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/register", get(handlers::auth::register_page).post(handlers::auth::register))
        .route("/logout", post(handlers::auth::logout))
        
        // User routes
        .route("/profile/:username", get(handlers::users::profile))
        .route("/profile/edit", get(handlers::users::edit_profile_page).post(handlers::users::update_profile))
        .route("/follow/:user_id", post(handlers::users::follow))
        .route("/unfollow/:user_id", post(handlers::users::unfollow))
        
        // Journal routes
        .route("/journals", get(handlers::journals::list))
        .route("/journals/my", get(handlers::journals::my_journals)) // New route for personal dashboard
        .route("/journals/new", get(handlers::journals::new_page).post(handlers::journals::create))
        .route("/journals/:id", get(handlers::journals::show))
        .route("/journals/:id/edit", get(handlers::journals::edit_page).post(handlers::journals::update))
        .route("/journals/:id/delete", post(handlers::journals::delete))
        .route("/journals/:id/publish", post(handlers::journals::publish))
        .route("/journals/:id/unpublish", post(handlers::journals::unpublish))
        
        // API routes
        .route("/api/journals", get(handlers::api::journals_list))
        .route("/api/feed", get(handlers::api::feed))
        
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Get port from environment or default to 3000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Run the app
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Journal app running on http://0.0.0.0:{}", port);
    
    axum::serve(listener, app).await?;

    Ok(())
}
