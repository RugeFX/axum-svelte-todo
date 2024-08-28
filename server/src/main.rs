use axum::{
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(Any);

    let app = Router::new().route("/", get(root).post(echo)).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn root() -> Json<User> {
    Json(User {
        id: 1,
        username: "RugeFX".to_owned(),
    })
}

async fn echo(body: String) -> String {
    match body {
        b if b.is_empty() => "bruh".to_owned(),
        _ => body,
    }
}
