use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{
    body::Body,
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use uuid::Uuid;

pub struct AppState {
    pub socket: SocketAddr,
    pub templates: Handlebars<'static>,
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
}
use handlebars::{DirectorySourceOptions, Handlebars};

impl AppState {
    pub async fn new() -> Self {
        let db_pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(
                std::env::var("DATABASE_URL")
                    .expect("No DATABASE_URL Specified in environment")
                    .as_str(),
            )
            .await
            .expect("Unable to establish Database connection pool");

        let host: IpAddr = IpAddr::from_str(
            std::env::var("HOST")
                .unwrap_or("0.0.0.0".to_string())
                .as_str(),
        )
        .unwrap();
        let port: u16 = std::env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse()
            .unwrap();
        let socket = SocketAddr::new(host, port);

        let mut templates = Handlebars::new();
        templates.set_dev_mode(cfg!(debug_assertions));
        templates
            .register_templates_directory("templates/", DirectorySourceOptions::default())
            .unwrap();

        AppState {
            socket,
            db_pool,
            templates,
        }
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Post {
    pub uuid: Uuid,
    pub slug: String,
    pub title: String,
    pub body: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub published: Option<time::OffsetDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostInsert {
    pub title: String,
    pub body: String,
}

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Unable to find post: {0}")]
    GetPost(sqlx::Error),
    #[error("Unable to get post list: {0}")]
    GetPostList(sqlx::Error),
    #[error("Error inserting new post: {0}")]
    NewPost(sqlx::Error),
    #[error("Error editing post: {0}")]
    EditPost(sqlx::Error),
    #[error("Error deleting post: {0}")]
    DeletePost(sqlx::Error),
    #[error("Error publishing post: {0}")]
    PublishPost(sqlx::Error),
}

pub struct WebResponse<'a> {
    status: StatusCode,
    template: &'a str,
    app_state: State<Arc<AppState>>,
    context: serde_json::Value,
}

impl<'a> WebResponse<'a> {
    pub fn new(template: &'a str, app_state: State<Arc<AppState>>) -> Self {
        Self {
            status: StatusCode::OK,
            template,
            app_state,
            context: json!({}),
        }
    }
    pub fn add_context(mut self, key: &str, value: impl Into<serde_json::Value>) -> Self {
        if let Some(context) = self.context.as_object_mut() {
            context.insert(key.to_string(), value.into());
        }
        self
    }
}

impl<'a> IntoResponse for WebResponse<'a> {
    fn into_response(self) -> axum::response::Response {
        let render = self
            .app_state
            .templates
            .render(self.template, &self.context)
            .unwrap();

        Response::builder()
            .status(self.status)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::new(render))
            .unwrap()
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        //let render = self
        //    .app_state
        //    .templates
        //    .render(self.template, &self.context)
        //    .unwrap();

        Response::builder()
            .status(500)
            .body(Body::new(self.to_string()))
            .unwrap()
    }
}
