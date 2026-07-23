use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::json;
use wyrite::AppState;

pub fn get_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/", get(view_home))
        .route("/posts", get(view_posts))
}

#[axum::debug_handler]
async fn view_home(app_state: State<Arc<AppState>>) -> impl IntoResponse {
    let data = &json!({
        "title": "Hello World",
        "body": "Welcome to wyrite"
    });

    app_state.templates.render("index", data).unwrap();
}

#[axum::debug_handler]
async fn view_posts(app_state: State<Arc<AppState>>) -> impl IntoResponse {
    let data = &json!({
        "title": "Index Page",
        "body": "Not yet ready"
    });

    app_state.templates.render("index", data).unwrap();
}
