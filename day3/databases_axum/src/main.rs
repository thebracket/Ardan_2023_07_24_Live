use axum::{routing::get, Router, Json, Extension};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build a connection pool and run migrations
    let pool = sqlx::SqlitePool::connect("sqlite:../databases/hello_db.db").await.unwrap();
    sqlx::migrate!("../databases/migrations")
        .run(&pool)
        .await
        .expect("Unable to migrate database");

    // Build a router, and add an extension layer containing the database pool
    let app = Router::new()
        .route("/", get(say_hello_json))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, FromRow)]
struct HelloJson {
    id: i64,
    message: String,
}

async fn say_hello_json(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<HelloJson>> {
    let result = sqlx::query_as::<_, HelloJson>("SELECT * FROM messages")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(result)
}