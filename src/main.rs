use std::sync::Arc;

use serde::{Deserialize, Serialize};

use axum::{extract::State, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use dotenvy::dotenv;

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
    sqlx::query("INSERT INTO posts (title, body) VALUES ($1,$2)")
        .bind("A Sample post")
        .bind("This is it's body")
        .execute(&db_pool)
        .await
        .expect("Couldn't add a post");

    let app = Router::new()
        .route("/", axum::routing::get(view_hw))
        .route("/posts", axum::routing::get(view_posts))
        .with_state(Arc::new(db_pool));

    let addr = std::env::var("HOST").unwrap_or("0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn view_hw() -> &'static str {
    "Hello from Axum, World!"
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
struct Post {
    title: String,
    body: String,
    is_published: bool,
    date_created: time::OffsetDateTime,
    date_published: Option<time::OffsetDateTime>,
}

async fn view_posts(State(db_p): State<Arc<Pool<Postgres>>>) -> String {
    let out = sqlx::query_as::<_, Post>("SELECT * FROM post_view")
        .fetch_all(&*db_p)
        .await
        .expect("couldn't query posts");

    serde_json::to_string_pretty(&out).unwrap()
}
