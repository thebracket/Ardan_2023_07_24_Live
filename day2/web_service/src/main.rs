use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::response::Html;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/html", get(say_hello_html))
        .route("/html_file", get(say_hello_file))
        .route("/json", get(say_hello_json))
        .route("/post", post(say_hello_post));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello_text() -> &'static str {
    "Hello, world!"
}

async fn say_hello_html() -> Html<&'static str> {
    const HTML: &str = include_str!("hello.html");
    Html(HTML)
}

async fn say_hello_file() -> Html<String> {
    let path = std::path::Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: "Hello, World!".to_string(),
    })
}

async fn say_hello_post() -> &'static str {
    "Hello, POST!"
}