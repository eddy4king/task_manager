mod db;
mod models;
mod handlers;

use dotenv::dotenv;
use axum::{routing::get, Router};
use handlers::health_check;


#[tokio::main]
async fn main() {
    dotenv().ok();
    db::connect().await;

    let app = Router::new()
            .route("/health", get (health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
    
