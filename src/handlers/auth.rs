use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use crate::models::User;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");

    let user = sqlx::query_as!(
    User,
    "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
    payload.username,
    payload.email,
    password_hash
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to create user");
    Json(json!({"user": user}))
}