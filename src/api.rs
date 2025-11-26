use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{Response, StatusCode},
    routing::get,
    Json,
};

use sqlx::{Pool, Postgres};

use uuid::Uuid;
use wyrite::{Post, PostInsert};

pub fn get_routes() -> axum::Router<Arc<Pool<Postgres>>> {
    axum::Router::new()
        .route("/posts", get(list_posts))
        .route("/post", get(view_post).post(add_post))
}

async fn list_posts(State(db_p): State<Arc<Pool<Postgres>>>) -> String {
    let out = sqlx::query_as!(Post, r#"SELECT * FROM post_view"#)
        .fetch_all(&*db_p)
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
    State(db_p): State<Arc<Pool<Postgres>>>, //State(db_p): State<Arc<Pool<Postgres>>>,
) -> Response<Body> {
    let post_query: Option<Post> = if let Some(slug) = page_opts.slug {
        sqlx::query_as!(Post, r#"SELECT * FROM post_view WHERE "slug!" = $1"#, slug)
            .fetch_optional(&*db_p)
            .await
            .expect("couldn't query posts")
    } else if let Some(uuid) = page_opts.uuid {
        sqlx::query_as!(
            Post,
            r#"SELECT * FROM post_view WHERE "uuid!" = $1::uuid"#,
            uuid
        )
        .fetch_optional(&*db_p)
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

async fn add_post(State(db_p): State<Arc<Pool<Postgres>>>, Json(post): Json<PostInsert>) -> String {
    sqlx::query!(
        r#"INSERT INTO posts (title, body) VALUES ($1,$2)"#,
        &post.title,
        &post.body
    )
    .execute(&*db_p)
    .await
    .expect("couldn't add a post")
    .rows_affected()
    .to_string()
}
