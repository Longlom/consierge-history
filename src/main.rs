use std::net::SocketAddr;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};

use dotenv::dotenv;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

mod save_message;
mod db;


use save_message::save_message;
use db::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = Database::create_connection().await;

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/messageHistory/v2.0/ping", get(ping))
        .route("/messageHistory/v2.0/save", post(save_message))
        .with_state(db)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener,  app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn ping() -> &'static str {
    "pong"
}
