use sqlx::{Row, FromRow};

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Fetch the messages from the database
    let messages = sqlx::query_as::<_, Message>("SELECT * FROM messages")
        .fetch_all(&pool)
        .await?;

    // Print the messages
    for message in messages {
        println!("{message:?}");
    }

    Ok(())
}
