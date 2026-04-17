# Task Manager API
# Task Manager API

A production-ready REST API built with Rust, featuring JWT authentication, PostgreSQL database, and full Docker support. This project was built from scratch as a deep dive into Rust backend development.

## Features

- User registration and login with JWT authentication
- Password hashing with bcrypt
- Full task CRUD operations (Create, Read, Update, Delete)
- Protected routes via JWT middleware
- PostgreSQL database with migrations
- Dockerized with Docker Compose
- GitHub Actions CI pipeline

## Tech Stack

- **Language** — Rust
- **Web Framework** — Axum
- **Database** — PostgreSQL
- **ORM/Query Builder** — SQLx
- **Authentication** — JWT (jsonwebtoken) + bcrypt
- **Runtime** — Tokio (async)
- **Containerization** — Docker + Docker Compose
- **CI** — GitHub Actions

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Docker](https://www.docker.com/) and Docker Compose
- [sqlx-cli](https://github.com/launchbadge/sqlx) (`cargo install sqlx-cli --no-default-features --features postgres`)

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/eddy4king/task_manager.git
cd task_manager
```

### 2. Create your environment file

```bash
cp .env.example .env
```

Or create `.env` manually:

DATABASE_URL=postgresql://postgres:password@localhost:5432/task_manager
JWT_SECRET=supersecretkey123

### 3. Start the database

```bash
docker compose up -d db
```

### 4. Run migrations

```bash
sqlx migrate run
```

### 5. Run the application

```bash
cargo run
```

The API will be available at `http://localhost:3000`.

### Running with Docker Compose (full stack)

```bash
docker compose up --build
```

This starts both the PostgreSQL database and the Rust API in containers.

## API Documentation

### Health Check

GET /health
Response:
```json
{ "status": "ok" }
```

---

### Register
POST /auth/register
Request body:
```json
{
  "username": "eddy",
  "email": "eddy@example.com",
  "password": "password123"
}
```

Response:
```json
{
  "user": {
    "id": "uuid",
    "username": "eddy",
    "email": "eddy@example.com",
    "created_at": "2026-04-15T00:00:00Z"
  }
}
```

---

### Login

POST /auth/login

Request body:
```json
{
  "email": "eddy@example.com",
  "password": "password123"
}
```

Response:
```json
{ "token": "eyJ0eXAiOiJKV1QiLC..." }
```

---

### Create Task

POST /tasks
Authorization: Bearer <token>
Request body:
```json
{
  "title": "My task",
  "description": "Task description",
  "priority": "high"
}
```

---

### Get All Tasks

GET /tasks
Authorization: Bearer <token>

---

### Get Single Task
GET /tasks/:id
Authorization: Bearer <token>

---

### Update Task

PUT /tasks/:id
Authorization: Bearer <token>

Request body:
```json
{
  "status": "completed",
  "priority": "low"
}
```

---

### Delete Task
DELETE /tasks/:id

Authorization: Bearer <token>
Response:
```json
{ "message": "Task deleted successfully" }
```

## Project Structure
task_manager/
├── .github/
│   └── workflows/
│       └── ci.yml           # GitHub Actions CI pipeline
├── migrations/
│   ├── 20260414093013_create_users_table.sql
│   └── 20260414094850_create_tasks_table.sql
├── src/
│   ├── main.rs              # Entry point, router, server
│   ├── db.rs                # Database connection pool
│   ├── state.rs             # Shared application state
│   ├── errors.rs            # Custom error types
│   ├── middleware.rs        # JWT auth extractor
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs          # User struct
│   │   └── task.rs          # Task struct
│   └── handlers/
│       ├── mod.rs
│       ├── health.rs        # Health check endpoint
│       ├── auth.rs          # Register and login endpoints
│       └── tasks.rs         # Task CRUD endpoints
├── Dockerfile               # Multi-stage Rust build
├── docker-compose.yml       # App + database containers
├── Cargo.toml               # Dependencies
└── .env                     # Environment variables (not committed)

## Development Steps

This project was built incrementally through the following phases:

1. **Project setup** — Cargo project initialised, Git repository created and pushed to GitHub
2. **Docker Compose** — PostgreSQL database container configured with named volume for data persistence
3. **Database connection** — SQLx connection pool established, environment variables loaded via dotenv
4. **Migrations** — `users` and `tasks` tables created with UUID primary keys, foreign keys, and timestamps
5. **Project structure** — Code separated into modules: `db`, `models`, `handlers`, `state`, `errors`, `middleware`
6. **Models** — `User` and `Task` structs defined with SQLx `FromRow` and Serde derive macros
7. **Web framework** — Axum router configured with shared application state
8. **Auth endpoints** — Register endpoint with bcrypt password hashing, login endpoint with JWT token generation
9. **JWT middleware** — Custom `AuthUser` extractor implementing `FromRequestParts` to protect routes
10. **Task CRUD** — Five task endpoints built with full JWT protection and proper SQL queries
11. **Error handling** — Custom `AppError` enum with `IntoResponse` and `From<sqlx::Error>` implementations replacing all panics
12. **Dockerfile** — Multi-stage build producing a minimal Debian runtime image
13. **CI pipeline** — GitHub Actions workflow running `cargo check` and `cargo clippy` on every push

## Environment Variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string |
| `JWT_SECRET` | Secret key for signing JWT tokens |

## License

MIT


