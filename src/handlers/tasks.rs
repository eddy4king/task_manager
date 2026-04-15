use axum::{extract::{State, Path}, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use crate::{errors::AppError, models::Task};
use crate::state::AppState;
use crate::middleware::AuthUser;
use chrono;


#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

pub async fn create_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = sqlx::query_as!(
    Task,
    "INSERT INTO tasks (user_id, title, description, priority, status, due_date)
     VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    auth_user.user_id,
    payload.title,
    payload.description,
    payload.priority.unwrap_or_else(|| "medium".to_string()),
    "pending",
    None::<chrono::DateTime<chrono::Utc>>
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Json(json!({"task": task})))
}

pub async fn get_tasks(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result< impl IntoResponse, AppError> {
    let tasks = sqlx::query_as!(
        Task,
        "SELECT * FROM tasks WHERE user_id = $1 ORDER BY created_at DESC",
        auth_user.user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({"tasks": tasks})))
}

pub async fn get_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = sqlx::query_as!(
        Task,
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2",
        id,
        auth_user.user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({"task": task})))
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
}

pub async fn update_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result< impl IntoResponse, AppError> {
    let task = sqlx::query_as!(
        Task,
        "UPDATE tasks SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            priority = COALESCE($3, priority),
            status = COALESCE($4, status)
         WHERE id = $5 AND user_id = $6
         RETURNING *",
        payload.title,
        payload.description,
        payload.priority,
        payload.status,
        id,
        auth_user.user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({"task": task})))
}

pub async fn delete_task(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result< impl IntoResponse, AppError> {
    sqlx::query!(
        "DELETE FROM tasks WHERE id = $1 AND user_id = $2",
        id,
        auth_user.user_id
    )
    .execute(&state.db)
    .await?;

    Ok(Json(json!({"message": "Task deleted successfully"})))
}