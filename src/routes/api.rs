use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{Response, StatusCode},
    routing::get,
    Json,
};
use uuid::Uuid;
use wyrite::{AppState, Post, PostInsert};

pub fn get_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/posts", get(list_posts))
        .route("/post", get(view_post).post(add_post))
}

async fn list_posts(State(app_state): State<Arc<AppState>>) -> String {
    let out = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(&app_state.db_pool)
        .await
        .expect("couldn't query posts");

    serde_json::to_string_pretty(&out).unwrap()
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ViewPostOpts {
    uuid: Option<Uuid>,
    slug: Option<String>,
}

async fn view_post(
    Query(page_opts): Query<ViewPostOpts>,
    State(app_state): State<Arc<AppState>>,
) -> Response<Body> {
    let post_query: Option<Post> = if let Some(slug) = page_opts.slug {
        sqlx::query_as!(Post, "SELECT * FROM posts WHERE slug = $1", slug)
            .fetch_optional(&app_state.db_pool)
            .await
            .expect("couldn't query posts")
    } else if let Some(uuid) = page_opts.uuid {
        sqlx::query_as!(Post, "SELECT * FROM posts WHERE uuid = $1::uuid", uuid)
            .fetch_optional(&app_state.db_pool)
            .await
            .expect("couldn't query posts")
    } else {
        None
    };

    if let Some(post) = post_query {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(serde_json::to_string_pretty(&post).unwrap()))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Post not found"))
            .unwrap()
    }
}

async fn add_post(State(app_state): State<Arc<AppState>>, Json(post): Json<PostInsert>) -> String {
    sqlx::query!(
        r#"INSERT INTO posts (title, body) VALUES ($1,$2)"#,
        &post.title,
        &post.body
    )
    .execute(&app_state.db_pool)
    .await
    .expect("couldn't add a post")
    .rows_affected()
    .to_string()
}
