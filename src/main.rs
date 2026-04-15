mod db;
mod models;
mod handlers;
mod state;
mod middleware;

use dotenv::dotenv;
use axum::{routing::{get, post, put, delete}, Router};
use handlers::{health_check, register, login, create_task, get_task, get_tasks, update_task, delete_task};
use state::AppState;




#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool= db::connect().await;
    let state = AppState{db: pool};

    let app = Router::new()
            .route("/health", get (health_check))
            .route("/auth/register", post(register))
            .route("/auth/login", post(login))
            .route("/tasks", post(create_task))
            .route("/tasks", get(get_tasks))
            .route("/tasks/:id", get(get_task))
            .route("/tasks/:id", put(update_task))
            .route("/tasks/:id", delete(delete_task))
            .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
    
