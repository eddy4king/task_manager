mod db;
mod models;
mod handlers;
mod state;

use dotenv::dotenv;
use axum::{routing::get, Router};
use handlers::health_check;
use state::AppState;



#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool= db::connect().await;
    let state = AppState{db: pool};

    let app = Router::new()
            .route("/health", get (health_check))
            .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
    
