# Journal App

> A minimalist journal platform built with Rust

## Features

- User authentication & profiles
- Rich text journal editor
- Draft, private, and public journals
- Follow other users
- Responsive design

## Quick Start

**GitHub Codespaces (Recommended)**
1. **Optional: Set up production database**
   - Create free database at [neon.tech](https://neon.tech)
   - Go to repo **Settings** → **Secrets** → **Codespaces**
   - Add `DATABASE_URL` and `JWT_SECRET` secrets
2. Click "Code" → "Codespaces" → "Create codespace"
3. Run setup: `chmod +x setup-codespace.sh && ./setup-codespace.sh`
4. Start app: `cargo run`
5. Access via forwarded port 3000

**Local Development**
```bash
# Clone and run
git clone https://github.com/AdityaDwiNugroho/journal-app.git
cd journal-app
cargo run
```

Visit `http://localhost:3000`

## Tech Stack

- **Rust** + Axum
- **PostgreSQL** with SQLx  
- **Askama** templates
- **JWT** authentication

## Deploy

**Railway + Neon (recommended)**

1. Create database at [neon.tech](https://neon.tech)
2. Deploy to [railway.app](https://railway.app)
3. Set `DATABASE_URL` and `JWT_SECRET` env vars

## License

MIT

## Author

Made by [Aditya Dwi Nugroho](https://teer.id/adityadwinugroho)
