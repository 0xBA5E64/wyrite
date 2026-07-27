use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::get;
use axum::Form;
use serde::Deserialize;
use serde_json::json;
use wyrite::AppState;

pub fn get_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(view_home))
        .route("/post/{slug}", get(view_post))
        .route("/post/{slug}/delete", get(delete_post))
        .route("/post/{slug}/publish", get(publish_post))
        .route("/posts", get(view_posts))
        .route("/posts/new", get(view_new_post).post(post_new_post))
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
    let post = sqlx::query_as!(wyrite::Post, "SELECT * FROM posts WHERE slug = $1", slug)
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
    let posts = sqlx::query_as!(wyrite::Post, "SELECT * FROM posts")
        .fetch_all(&app_state.db_pool)
        .await
        .unwrap();

    let data = &json!({
        "posts": posts
    });

    Html(app_state.templates.render("posts", data).unwrap())
}

async fn view_new_post(app_state: State<Arc<AppState>>) -> impl IntoResponse {
    Html(app_state.templates.render("new_post", &json!({})).unwrap())
}

#[derive(Deserialize)]
struct NewPost {
    title: String,
    body: String,
}

#[axum::debug_handler]
async fn post_new_post(
    app_state: State<Arc<AppState>>,
    Form(new_post): Form<NewPost>,
) -> impl IntoResponse {
    let new_post = sqlx::query!(
        "INSERT INTO Posts (title, body) VALUES ($1, $2) RETURNING slug",
        new_post.title,
        new_post.body
    )
    .fetch_one(&app_state.db_pool)
    .await;

    match new_post {
        Ok(slug) => Redirect::to(format!("/post/{}", slug.slug).as_str()).into_response(),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("{err:?}")))
            .unwrap(),
    }
}

#[axum::debug_handler]
async fn delete_post(
    app_state: State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let query = sqlx::query!("DELETE FROM Posts WHERE slug = $1 RETURNING slug", slug)
        .fetch_optional(&app_state.db_pool)
        .await
        .unwrap();

    match query {
        Some(_) => Redirect::to("/posts").into_response(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    }
}

#[axum::debug_handler]
async fn publish_post(
    app_state: State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let query = sqlx::query!(
        "UPDATE Posts SET published = current_timestamp(0) WHERE slug = $1 RETURNING slug",
        slug
    )
    .fetch_optional(&app_state.db_pool)
    .await
    .unwrap();

    match query {
        Some(_) => Redirect::to("/posts").into_response(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    }
}
