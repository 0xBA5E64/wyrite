#![warn(clippy::pedantic)]
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

#[derive(Error, Debug)]
pub enum AppStateError {
    #[error("Unable to obtain DATABASE_URL from enviroument: {0}")]
    DatabaseConnectionUrl(std::env::VarError),
    #[error("Unable to establish Database connection pool: {0}")]
    DatabaseConnection(sqlx::Error),
    #[error("Unable to parse specified HOST as valid IP: {0}")]
    HostParse(std::net::AddrParseError),
    #[error("Unable to parse specified POST as valid integer: {0}")]
    PortParse(std::num::ParseIntError),
    #[error("Unable to register Handlebars templates: {0}")]
    HandlebarsRegistration(handlebars::TemplateError),
}

pub struct AppState {
    pub socket: SocketAddr,
    pub templates: Handlebars<'static>,
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
}
use handlebars::{DirectorySourceOptions, Handlebars};

#[allow(clippy::missing_errors_doc)]
impl AppState {
    pub async fn new() -> Result<Self, AppStateError> {
        let db_pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(
                std::env::var("DATABASE_URL")
                    .map_err(AppStateError::DatabaseConnectionUrl)?
                    .as_str(),
            )
            .await
            .map_err(AppStateError::DatabaseConnection)?;

        let host: IpAddr = IpAddr::from_str(
            std::env::var("HOST")
                .unwrap_or("0.0.0.0".to_string())
                .as_str(),
        )
        .map_err(AppStateError::HostParse)?;
        let port: u16 = std::env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse()
            .map_err(AppStateError::PortParse)?;
        let socket = SocketAddr::new(host, port);

        let mut templates = Handlebars::new();
        templates.set_dev_mode(cfg!(debug_assertions));
        templates
            .register_templates_directory("templates/", DirectorySourceOptions::default())
            .map_err(AppStateError::HandlebarsRegistration)?;

        Ok(AppState {
            socket,
            templates,
            db_pool,
        })
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
    /// Construct a new `WebResponse` instance.
    /// Use as a builder and call `.set_status()` / `.add_context()` to flesh out the response.
    #[must_use]
    pub fn new(template: &'a str, app_state: State<Arc<AppState>>) -> Self {
        Self {
            status: StatusCode::OK,
            template,
            app_state,
            context: json!({}),
        }
    }
    /// Set the response status code to something other than the default 200
    #[must_use]
    pub fn set_status(mut self, status_code: StatusCode) -> Self {
        self.status = status_code;
        self
    }
    /// Append additional context to the response for use in the template
    #[must_use]
    pub fn add_context(mut self, key: &str, value: impl Into<serde_json::Value>) -> Self {
        if let Some(context) = self.context.as_object_mut() {
            context.insert(key.to_string(), value.into());
        }
        self
    }
    /// Shortcut method for constructing error response
    #[must_use]
    pub fn new_error(app_state: State<Arc<AppState>>, error: &WebError) -> Self {
        Self::new("error", app_state)
            .set_status(StatusCode::INTERNAL_SERVER_ERROR)
            .add_context("err_msg", json!(error.to_string()))
    }
}

impl IntoResponse for WebResponse<'_> {
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
