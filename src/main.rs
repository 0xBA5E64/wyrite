#![warn(clippy::pedantic)]
use std::sync::Arc;

use axum::Router;
use dotenvy::dotenv;
use tower_http::services::ServeDir;
use wyrite::AppState;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state = Arc::new(AppState::new().await.unwrap());

    // Run Database migrations
    sqlx::migrate!()
        .run(&app_state.db_pool)
        .await
        .expect("Migrations Failed");

    let listener = tokio::net::TcpListener::bind(&app_state.socket)
        .await
        .unwrap();

    let app = Router::new()
        .merge(routes::web::get_routes())
        .nest("/api", routes::api::get_routes())
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(app_state);

    println!("Serving to http://{}/", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
