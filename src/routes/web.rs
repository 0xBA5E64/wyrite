#![warn(clippy::pedantic)]
use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Form,
};
use wyrite::{AppState, PostInsert, WebError};

use crate::routes::web_templates;

pub fn get_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(view_home))
        .route("/post/{slug}", get(view_post))
        .route("/post/{slug}/delete", get(delete_post))
        .route("/post/{slug}/publish", get(publish_post))
        .route("/post/{slug}/edit", get(edit_post).post(post_edit_post))
        .route("/posts", get(view_posts))
        .route("/posts/new", get(edit_new_post).post(post_new_post))
}

#[axum::debug_handler]
async fn view_home() -> impl IntoResponse {
    Html(
        web_templates::Index {
            title: "Hello from Askama",
            body: "This is a Askama template",
        }
        .render()
        .unwrap(),
    )
}

#[axum::debug_handler]
async fn view_post(app_state: State<Arc<AppState>>, Path(slug): Path<String>) -> impl IntoResponse {
    let query = sqlx::query_as!(wyrite::Post, "SELECT * FROM posts WHERE slug = $1", slug)
        .fetch_optional(&app_state.db_pool)
        .await
        .map_err(WebError::GetPost);

    match query {
        Ok(query) => match query {
            Some(post) => Html(web_templates::Post { post: &post }.render().unwrap()),
            None => Html(
                web_templates::NotFound {
                    error: &format!("Post \"{slug}\" not found"),
                }
                .render()
                .unwrap(),
            ),
        },
        Err(error) => Html(web_templates::Error { error: &error }.render().unwrap()),
    }
}

#[axum::debug_handler]
async fn view_posts(app_state: State<Arc<AppState>>) -> impl IntoResponse {
    let query = sqlx::query_as!(wyrite::Post, "SELECT * FROM posts")
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(WebError::GetPostList);

    match query {
        Ok(posts) => Html(web_templates::Posts { posts: &posts }.render().unwrap()),
        Err(error) => Html(web_templates::Error { error: &error }.render().unwrap()),
    }
}

async fn edit_new_post() -> impl IntoResponse {
    Html(
        web_templates::EditPost {
            post: &Option::None,
        }
        .render()
        .unwrap(),
    )
}

#[axum::debug_handler]
async fn post_new_post(
    app_state: State<Arc<AppState>>,
    Form(new_post): Form<PostInsert>,
) -> impl IntoResponse {
    let query = sqlx::query!(
        "INSERT INTO Posts (title, body) VALUES ($1, $2) RETURNING slug",
        new_post.title,
        new_post.body
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(WebError::NewPost);

    match query {
        Ok(new_post) => Redirect::to(format!("/post/{}", new_post.slug).as_str()).into_response(),
        Err(error) => Html(web_templates::Error { error: &error }.render().unwrap()).into_response(),
    }
}

async fn edit_post(app_state: State<Arc<AppState>>, Path(slug): Path<String>) -> impl IntoResponse {
    let query = sqlx::query_as!(wyrite::Post, "SELECT * FROM posts WHERE slug = $1", slug)
        .fetch_optional(&app_state.db_pool)
        .await
        .map_err(WebError::EditPost);

    match query {
        Ok(query) => match query {
            Some(post) => Html(
                web_templates::EditPost {
                    post: &Option::Some(post),
                }
                .render()
                .unwrap(),
            ),
            None => Html(
                web_templates::NotFound {
                    error: &format!("Post \"{slug}\" not found"),
                }.render().unwrap()),
        },
        Err(error) => Html(web_templates::Error { error: &error }.render().unwrap()),
    }
}

#[axum::debug_handler]
async fn post_edit_post(
    app_state: State<Arc<AppState>>,
    Path(slug): Path<String>,
    Form(new_post): Form<PostInsert>,
) -> impl IntoResponse {
    let query = sqlx::query!(
        "UPDATE Posts SET title = $1, body = $2 WHERE slug = $3 RETURNING slug",
        new_post.title,
        new_post.body,
        slug
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(WebError::EditPost);
    // TODO: Handle posts not being valid
    // TODO-TODO: Post content validation.
    match query {
        Ok(new_post) => Redirect::to(format!("/post/{}", new_post.slug).as_str()).into_response(),
        Err(error) => Html(web_templates::Error { error: &error }.render().unwrap()).into_response(),
    }
}

#[axum::debug_handler]
async fn delete_post(
    app_state: State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let query = sqlx::query!("DELETE FROM Posts WHERE slug = $1", slug)
        .execute(&app_state.db_pool)
        .await
        .map_err(WebError::DeletePost);

    match query {
        Ok(_) => Redirect::to("/posts").into_response(),
        Err(error) => Html(web_templates::Error { error: &error }.render().unwrap()).into_response(),
    }
}

#[axum::debug_handler]
async fn publish_post(
    app_state: State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let query = sqlx::query!(
        "UPDATE Posts SET published = current_timestamp(0) WHERE slug = $1",
        slug
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(WebError::PublishPost);

    match query {
        Ok(_) => Redirect::to("/posts").into_response(),
        Err(error) => Html(web_templates::Error { error: &error }.render().unwrap()).into_response(),
    }
}
