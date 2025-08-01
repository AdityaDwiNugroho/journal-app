use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

pub async fn run_migrations(_pool: &PgPool) -> Result<()> {
    // Create tables manually for now since we're not using SQLx migrations
    // Execute each statement separately to avoid multi-statement issues
    
    // Create users table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(50) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            display_name VARCHAR(100),
            bio TEXT,
            is_private BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#).execute(_pool).await?;

    // Create journals table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS journals (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            is_published BOOLEAN DEFAULT FALSE,
            is_private BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            published_at TIMESTAMP WITH TIME ZONE
        )
    "#).execute(_pool).await?;

    // Create follows table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS follows (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            following_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            UNIQUE(follower_id, following_id)
        )
    "#).execute(_pool).await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_journals_user_id ON journals(user_id)")
        .execute(_pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_journals_published ON journals(is_published)")
        .execute(_pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_journals_published_at ON journals(published_at)")
        .execute(_pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_follows_follower_id ON follows(follower_id)")
        .execute(_pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_follows_following_id ON follows(following_id)")
        .execute(_pool).await?;

    // Create updated_at trigger function
    sqlx::query(r#"
        CREATE OR REPLACE FUNCTION update_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        $$ language 'plpgsql'
    "#).execute(_pool).await?;

    // Create triggers for updated_at (handle if they exist)
    let _ = sqlx::query("DROP TRIGGER IF EXISTS update_users_updated_at ON users")
        .execute(_pool).await;
    sqlx::query("CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column()")
        .execute(_pool).await?;
    
    let _ = sqlx::query("DROP TRIGGER IF EXISTS update_journals_updated_at ON journals")
        .execute(_pool).await;
    sqlx::query("CREATE TRIGGER update_journals_updated_at BEFORE UPDATE ON journals FOR EACH ROW EXECUTE FUNCTION update_updated_at_column()")
        .execute(_pool).await?;
    
    println!("✅ Database tables created successfully");
    Ok(())
}
