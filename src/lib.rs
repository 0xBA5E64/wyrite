use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub date_published: Option<time::OffsetDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostInsert {
    pub title: String,
    pub body: String,
}
