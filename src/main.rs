use std::sync::Arc;

use serde::{Deserialize, Serialize};

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use uuid::Uuid;

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
        .route("/posts", get(list_posts))
        .route("/posts/{post_id}", get(view_post))
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
    uuid: Uuid,
    title: String,
    body: String,
    is_published: bool,
    #[serde(with = "time::serde::rfc3339")]
    date_created: time::OffsetDateTime,
    // TODO: Serialize this too with time::serde::rfc3339, tricky since Option :|
    date_published: Option<time::OffsetDateTime>,
}

async fn list_posts(State(db_p): State<Arc<Pool<Postgres>>>) -> String {
    // This query is a total mess due to sqlx being unable to determine the nullability of fields of a view
    // See https://github.com/launchbadge/sqlx/issues/3192#issuecomment-2807790647
    let out = sqlx::query_as!(
        Post,
        r#"
            SELECT
                uuid AS "uuid!",
                title AS "title!",
                body AS "body!",
                is_published AS "is_published!",
                date_created AS "date_created!",
                date_published FROM post_view"#
    )
    .fetch_all(&*db_p)
    .await
    .expect("couldn't query posts");

    serde_json::to_string_pretty(&out).unwrap()
}

async fn view_post(Path(post_id): Path<Uuid>, State(db_p): State<Arc<Pool<Postgres>>>) -> String {
    let out = sqlx::query_as!(
        Post,
        r#"
            SELECT
                uuid AS "uuid!",
                title AS "title!",
                body AS "body!",
                is_published AS "is_published!",
                date_created AS "date_created!",
                date_published FROM post_view
            WHERE uuid = $1::uuid"#,
        post_id
    )
    .fetch_one(&*db_p)
    .await
    .expect("couldn't query posts");

    serde_json::to_string_pretty(&out).unwrap()
}
