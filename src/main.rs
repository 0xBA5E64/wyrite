use std::sync::Arc;

use axum::{routing::get, Router};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

mod api;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let db_pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(
            std::env::var("DATABASE_URL")
                .expect("No DATABASE_URL Specified in environment")
                .as_str(),
        )
        .await
        .expect("no bueno deebee");

    // Run Database migrations
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Migrations Failed");

    // Insert a sample post
    sqlx::query!(
        "INSERT INTO posts (title, body) VALUES ($1,$2)",
        "A Sample post",
        "This is it's body"
    )
    .execute(&db_pool)
    .await
    .expect("Couldn't add a post");

    let app = Router::new()
        .route("/", get(view_hw))
        .nest("/api", api::get_routes())
        .with_state(Arc::new(db_pool));

    let addr = std::env::var("HOST").unwrap_or("0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn view_hw() -> &'static str {
    "Hello from Axum, World!"
}
