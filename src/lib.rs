use std::{net::Ipv4Addr, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub host: Ipv4Addr,
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
}

impl AppState {
    pub async fn new() -> Self {
        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect(
                std::env::var("DATABASE_URL")
                    .expect("No DATABASE_URL Specified in environment")
                    .as_str(),
            )
            .await
            .expect("no bueno deebee");

        let host: Ipv4Addr = Ipv4Addr::from_str(
            std::env::var("HOST")
                .unwrap_or("0.0.0.0".to_string())
                .as_str(),
        )
        .unwrap();

        AppState { db_pool, host }
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Post {
    pub uuid: Uuid,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub is_published: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub date_created: time::OffsetDateTime,
    // TODO: Serialize this too with time::serde::rfc3339, tricky since Option :|
    #[serde(with = "time::serde::rfc3339::option")]
    pub date_published: Option<time::OffsetDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostInsert {
    pub title: String,
    pub body: String,
}
