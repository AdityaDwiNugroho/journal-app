# Journal App

> A minimalist journal platform built with Rust

## Features

- User authentication & profiles
- Rich text journal editor
- Draft, private, and public journals
- Follow other users
- Responsive design

## Quick Start

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
