use axum::extract::Path;
use axum::{routing::get, Router, Json, Extension};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use std::net::SocketAddr;
use std::collections::HashMap;

struct MessageCache {
    messages: HashMap<i64, HelloJson>,
}

impl MessageCache {
    fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    async fn get(&mut self, id: i64, pool: &sqlx::SqlitePool) -> Option<HelloJson> {
        // Do we have a cached entry?
        if let Some(msg) = self.messages.get(&id) {
            // Yes - return it
            Some(msg.clone())
        } else {
            // No - look it up in the database
            let row = sqlx::query_as::<_, HelloJson>("SELECT * FROM messages WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await;
            if let Ok(row) = row {
                self.messages.insert(row.id, row.clone());
                Some(row)
            } else {
                None
            }
        }
    }
}

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
        .route("/one/:id", get(get_one))
        .layer(Extension(pool))
        .layer(Extension(
            std::sync::Arc::new(
                tokio::sync::Mutex::new(
                    MessageCache::new()
                )
            )
        ));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
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

type MessagesCache = std::sync::Arc<tokio::sync::Mutex<MessageCache>>;

async fn get_one(
    Path(id): Path<i64>,
    Extension(pool): Extension<sqlx::SqlitePool>,
    Extension(cache): Extension<MessagesCache>,
) -> Json<HelloJson> {
    let mut lock = cache.lock().await;
    Json(lock.get(id, &pool).await.unwrap())
}