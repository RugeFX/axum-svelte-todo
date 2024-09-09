use std::env;

use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{delete, get},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::{Any, CorsLayer};

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
        .route("/api/todos", get(get_todos).post(post_todo))
        .route("/api/todos/:id", delete(delete_todo).get(get_todo_by_id))
        .layer(cors)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("App listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Todo {
    id: i32,
    title: String,
    body: String,
}

#[derive(Deserialize, Debug)]
struct CreateTodo {
    title: String,
    body: String,
}

#[derive(Serialize)]
struct CreateError {
    status: String,
    message: String,
}

async fn html() -> Html<&'static str> {
    Html("<h1>HTML thingymajiggy</h1>")
}

async fn root() -> Json<Todo> {
    Json(Todo {
        id: 1,
        title: "Title".to_owned(),
        body: "Body".to_owned(),
    })
}

async fn echo(body: String) -> String {
    match body {
        b if b.is_empty() => "bruh".to_owned(),
        _ => body,
    }
}

async fn get_todos(State(db): State<PgPool>) -> Json<Vec<Todo>> {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&db)
        .await
        .unwrap();

    Json(result)
}

async fn get_todo_by_id(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, (StatusCode, Json<CreateError>)> {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&db)
        .await;

    match result {
        Err(err) => {
            println!("{err}");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateError {
                    status: "failed".to_owned(),
                    message: "I don't know why but it failed :(".to_owned(),
                }),
            ))
        }
        Ok(result) => Ok(Json(result)),
    }
}

async fn post_todo(
    State(db): State<PgPool>,
    Json(body): Json<CreateTodo>,
) -> Result<impl IntoResponse, (StatusCode, Json<CreateError>)> {
    println!("{body:?}");

    let result = sqlx::query("INSERT INTO todos (title, body) VALUES ($1, $2)")
        .bind(body.title)
        .bind(body.body)
        .execute(&db)
        .await;

    match result {
        Err(err) => {
            println!("{err}");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateError {
                    status: "failed".to_owned(),
                    message: "I don't know why but it failed :(".to_owned(),
                }),
            ))
        }
        Ok(result) => Ok(format!("{:?}", result)),
    }
}

async fn delete_todo(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<CreateError>)> {
    println!("{id}");

    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await;

    match result {
        Err(err) => {
            println!("{err}");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateError {
                    status: "failed".to_owned(),
                    message: "Failed to delete todo :(".to_owned(),
                }),
            ))
        }
        Ok(result) => {
            if result.rows_affected() == 0 {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(CreateError {
                        status: "failed".to_owned(),
                        message: format!("Todo with id {} not found! :(", id),
                    }),
                ))
            } else {
                Ok(format!("{:?}", result))
            }
        }
    }
}
