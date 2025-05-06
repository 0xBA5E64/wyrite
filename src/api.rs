use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
};

use sqlx::{Pool, Postgres};

use uuid::Uuid;
use wyrite::Post;

pub fn get_routes() -> axum::Router<Arc<Pool<Postgres>>> {
    axum::Router::new()
        .route("/posts", get(list_posts))
        .route("/posts/{post_id}", get(view_post))
}

async fn list_posts(State(db_p): State<Arc<Pool<Postgres>>>) -> String {
    let out = sqlx::query_as!(Post, r#"SELECT * FROM post_view"#)
        .fetch_all(&*db_p)
        .await
        .expect("couldn't query posts");

    serde_json::to_string_pretty(&out).unwrap()
}

async fn view_post(Path(post_id): Path<Uuid>, State(db_p): State<Arc<Pool<Postgres>>>) -> String {
    let out = sqlx::query_as!(
        Post,
        r#"SELECT * FROM post_view WHERE "uuid!" = $1::uuid"#,
        post_id
    )
    .fetch_one(&*db_p)
    .await
    .expect("couldn't query posts");

    serde_json::to_string_pretty(&out).unwrap()
}
