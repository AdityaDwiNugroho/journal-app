#!/bin/bash

echo "🚀 Setting up Journal App in Codespace..."

# Check if we have production database URL from secrets
if [ -n "$DATABASE_URL" ]; then
    echo "Using production database from GitHub secrets"
    echo "Using JWT secret from GitHub secrets"
    
    # Add to .bashrc for persistence
    echo "export DATABASE_URL=\"$DATABASE_URL\"" >> ~/.bashrc
    echo "export JWT_SECRET=\"${JWT_SECRET:-your-fallback-jwt-secret}\"" >> ~/.bashrc
    echo "export PORT=\"3000\"" >> ~/.bashrc
    
    echo "Production environment ready!"
    echo "🏃 Run: cargo run"
    echo "🌐 Your app will be available on the forwarded port 3000"
    exit 0
fi

echo "No production secrets found, setting up local development database..."

# Install PostgreSQL if not available
if ! command -v psql &> /dev/null; then
    echo "Installing PostgreSQL..."
    sudo apt-get update
    sudo apt-get install -y postgresql postgresql-contrib
fi

# Start PostgreSQL service
echo "🗄️ Starting PostgreSQL..."
sudo service postgresql start

# Create database and user
echo "Setting up local database..."
sudo -u postgres psql -c "CREATE DATABASE journal_app;" 2>/dev/null || echo "Database already exists"
sudo -u postgres psql -c "CREATE USER journal_user WITH PASSWORD 'password';" 2>/dev/null || echo "User already exists"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE journal_app TO journal_user;" 2>/dev/null

# Set environment variables
echo "Setting local environment variables..."
export DATABASE_URL="postgresql://journal_user:password@localhost/journal_app"
export JWT_SECRET="your-super-secret-jwt-key-for-codespaces"
export PORT="3000"

# Add to .bashrc for persistence
echo 'export DATABASE_URL="postgresql://journal_user:password@localhost/journal_app"' >> ~/.bashrc
echo 'export JWT_SECRET="your-super-secret-jwt-key-for-codespaces"' >> ~/.bashrc
echo 'export PORT="3000"' >> ~/.bashrc

echo "Setup complete!"
echo "🏃 Run: cargo run"
echo "🌐 Your app will be available on the forwarded port 3000"
