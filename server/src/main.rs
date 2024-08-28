use std::env;

use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::fmt::format;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not detected!");
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

    let db_url = env::var("DATABASE_URL")?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db)
        .await?;

    assert_eq!(row.0, 150);

    let app = Router::new()
        .route("/", get(html))
        .route("/api", get(root).post(echo))
        .route("/query", get(query_test))
        .layer(cors)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("App listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn query_test(State(db): State<PgPool>) -> String {
    let result = sqlx::query("SELECT * FROM todos")
        .fetch_all(&db)
        .await
        .unwrap();

    format!("Todo : {:?}", result)
}

async fn html() -> Html<&'static str> {
    Html("<h1>HTML thingymajiggy</h1>")
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
