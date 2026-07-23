use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use serde_json::json;
use wyrite::AppState;

pub fn get_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(view_home))
        .route("/post/{slug}", get(view_post))
        .route("/posts", get(view_posts))
}

#[axum::debug_handler]
async fn view_home(app_state: State<Arc<AppState>>) -> impl IntoResponse {
    let data = &json!({
        "title": "Hello World",
        "body": "Welcome to wyrite"
    });

    Html(app_state.templates.render("index", data).unwrap())
}

#[axum::debug_handler]
async fn view_post(app_state: State<Arc<AppState>>, Path(slug): Path<String>) -> impl IntoResponse {
    let post = sqlx::query_as!(
        wyrite::Post,
        "SELECT * FROM post_view WHERE \"slug!\" = $1",
        slug
    )
    .fetch_optional(&app_state.db_pool)
    .await
    .unwrap();

    let data = &json!({
        "post": post
    });

    Html(app_state.templates.render("post", data).unwrap())
}

#[axum::debug_handler]
async fn view_posts(app_state: State<Arc<AppState>>) -> impl IntoResponse {
    let posts = sqlx::query_as!(wyrite::Post, "SELECT * FROM post_view")
        .fetch_all(&app_state.db_pool)
        .await
        .unwrap();

    let data = &json!({
        "posts": posts
    });

    Html(app_state.templates.render("posts", data).unwrap())
}
