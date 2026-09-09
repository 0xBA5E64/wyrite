#![warn(clippy::pedantic)]
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
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
}

pub struct AppState {
    pub socket: SocketAddr,
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
}

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

        Ok(AppState { socket, db_pool })
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
