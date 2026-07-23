use std::sync::Arc;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use wyrite::AppState;

mod api;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let app_state = Arc::new(AppState::new().await);

    // Run Database migrations
    sqlx::migrate!()
        .run(&app_state.db_pool)
        .await
        .expect("Migrations Failed");

    let host = &app_state.host.clone();

    let app = Router::new()
        .route("/", get(view_hw))
        .nest("/api", api::get_routes())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn view_hw() -> &'static str {
    "Hello from Axum, World!"
}
