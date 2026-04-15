use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::models::User;
use crate::state::AppState;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
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
    .await?;
    Ok(Json(json!({"user": user})))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub sub: String,
    pub exp: usize,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&state.db)
    .await?;

    let valid = bcrypt::verify(&payload.password, &user.password_hash)
        .expect("Failed to verify password");
    if !valid {
        return Err(AppError::Unauthorized);
    }
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: user.id.to_string(),
        exp: 100000000000,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to generate token");

     Ok(Json(json!({"token": token})))
}