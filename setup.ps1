# Database setup script for Journal App (Windows PowerShell)

Write-Host "🚀 Journal App Database Setup" -ForegroundColor Green
Write-Host "=================================="

# Check if PostgreSQL is installed
$psqlPath = Get-Command psql -ErrorAction SilentlyContinue
if (-not $psqlPath) {
    Write-Host "❌ PostgreSQL is not installed. Please install PostgreSQL first." -ForegroundColor Red
    Write-Host "Download from: https://www.postgresql.org/download/"
    exit 1
}

Write-Host "✅ PostgreSQL found" -ForegroundColor Green

# Create database
Write-Host "📦 Creating database 'journal_app'..." -ForegroundColor Yellow
try {
    createdb journal_app 2>$null
    Write-Host "✅ Database 'journal_app' created" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Database 'journal_app' might already exist" -ForegroundColor Yellow
}

# Create .env file if it doesn't exist
if (-not (Test-Path .env)) {
    Write-Host "📝 Creating .env file..." -ForegroundColor Yellow
    Copy-Item .env.example .env
    Write-Host "✅ Created .env file. Please update the DATABASE_URL and JWT_SECRET." -ForegroundColor Green
} else {
    Write-Host "✅ .env file already exists" -ForegroundColor Green
}

# Check for Rust and Cargo
$cargoPath = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoPath) {
    Write-Host "❌ Rust/Cargo is not installed. Please install Rust first." -ForegroundColor Red
    Write-Host "Download from: https://rustup.rs/"
    exit 1
}

Write-Host "✅ Rust/Cargo found" -ForegroundColor Green

# Install SQLx CLI if not present
$sqlxPath = Get-Command sqlx -ErrorAction SilentlyContinue
if (-not $sqlxPath) {
    Write-Host "📦 Installing SQLx CLI..." -ForegroundColor Yellow
    cargo install sqlx-cli --no-default-features --features postgres
} else {
    Write-Host "✅ SQLx CLI found" -ForegroundColor Green
}

Write-Host ""
Write-Host "🎉 Setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:"
Write-Host "1. Update your .env file with the correct database credentials"
Write-Host "2. Run 'cargo run' to start the application"
Write-Host "3. Visit http://localhost:3000 in your browser"
Write-Host ""
Write-Host "Note: The application will automatically run database migrations on startup." -ForegroundColor Yellow
