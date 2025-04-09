use std::sync::Arc;

use time;

use axum::{extract::State, Router};
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteRow},
    FromRow, Pool, Row, Sqlite,
};

#[tokio::main]
async fn main() {
    let db_pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect("sqlite://db.sqlite?mode=rwc")
        .await
        .expect("no bueno deebee");

    // Run Database migrations
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Migrations Failed");

    // Insert a sample post
    sqlx::query("INSERT INTO posts (title, body, is_published, date_created) VALUES (?,?,?,?)")
        .bind("A Sample post")
        .bind("This is it's body")
        .bind(0)
        .bind(time::UtcDateTime::now().unix_timestamp())
        .execute(&db_pool)
        .await
        .expect("Couldn't add a post");

    let app = Router::new()
        .route("/", axum::routing::get(view_hw))
        .route("/posts", axum::routing::get(view_posts))
        .with_state(Arc::new(db_pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn view_hw() -> &'static str {
    "Hello from Axum, World!"
}

#[derive(Debug)]
struct Post {
    title: String,
    body: String,
    is_published: bool,
    date_created: time::UtcDateTime,
}

impl FromRow<'_, SqliteRow> for Post {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self, sqlx::Error> {
        Ok(Self {
            title: row.try_get("title")?,
            body: row.try_get("body")?,
            is_published: row.try_get("is_published")?,
            date_created: time::UtcDateTime::from_unix_timestamp(row.try_get("date_created")?)
                .unwrap(),
        })
    }
}

async fn view_posts(State(db_p): State<Arc<Pool<Sqlite>>>) -> String {
    let out = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&*db_p)
        .await
        .expect("couldn't query posts");

    format!("{:?}", out).to_string()
}
