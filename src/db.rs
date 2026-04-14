use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn connect() ->PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to : {}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    println!("Database connected successfully!");
    pool
}