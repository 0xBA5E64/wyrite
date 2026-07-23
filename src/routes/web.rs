use std::sync::Arc;

use axum::routing::get;
use wyrite::AppState;

pub fn get_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(view_home))
        .route("/posts", get(view_posts))
}

async fn view_home() -> &'static str {
    "Hello from Axum, World!"
}

async fn view_posts() -> &'static str {
    "This will be the posts list!"
}
