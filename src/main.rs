use std::sync::Arc;

use axum::Router;
use dotenvy::dotenv;
use tower_http::services::ServeDir;
use wyrite::AppState;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let app_state = Arc::new(AppState::new().await);

    // Run Database migrations
    sqlx::migrate!()
        .run(&app_state.db_pool)
        .await
        .expect("Migrations Failed");

    let host = app_state.host;
    let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();

    let app = Router::new()
        .merge(routes::web::get_routes())
        .nest("/api", routes::api::get_routes())
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
