#!/bin/bash

# Database setup script for Journal App

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Journal App Database Setup${NC}"
echo "=================================="

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo -e "${RED}❌ PostgreSQL is not installed. Please install PostgreSQL first.${NC}"
    echo "Download from: https://www.postgresql.org/download/"
    exit 1
fi

echo -e "${GREEN}✅ PostgreSQL found${NC}"

# Create database
echo -e "${YELLOW}📦 Creating database 'journal_app'...${NC}"
createdb journal_app 2>/dev/null || echo -e "${YELLOW}⚠️  Database 'journal_app' might already exist${NC}"

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo -e "${YELLOW}📝 Creating .env file...${NC}"
    cp .env.example .env
    echo -e "${GREEN}✅ Created .env file. Please update the DATABASE_URL and JWT_SECRET.${NC}"
else
    echo -e "${GREEN}✅ .env file already exists${NC}"
fi

# Install SQLx CLI if not present
if ! command -v sqlx &> /dev/null; then
    echo -e "${YELLOW}📦 Installing SQLx CLI...${NC}"
    cargo install sqlx-cli --no-default-features --features postgres
else
    echo -e "${GREEN}✅ SQLx CLI found${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Setup complete!${NC}"
echo ""
echo "Next steps:"
echo "1. Update your .env file with the correct database credentials"
echo "2. Run 'cargo run' to start the application"
echo "3. Visit http://localhost:3000 in your browser"
echo ""
echo -e "${YELLOW}Note: The application will automatically run database migrations on startup.${NC}"
