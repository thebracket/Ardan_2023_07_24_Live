#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Hello World!");
    tracing::warn!("Hello World!");
    tracing::error!("Hello World!");
}