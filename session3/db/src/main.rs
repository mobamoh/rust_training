// use sqlx::{prelude::FromRow, Row};

use futures::{future::ok, TryStreamExt};
use sqlx::prelude::FromRow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // sqlx::migrate!("./migrations").run(&pool).await?;
    /*
    let messages = sqlx::query("SELECT id, message FROM messages")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let id: i64 = row.get(0);
            let message: String = row.get(1);
            (id, message)
        })
        .fetch_all(&pool)
        .await?;

    for (id, msg) in messages {
        println!("{id}: {msg}");
    }
    */

    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch_all(&pool)
        .await?;

    for msg in messages {
        println!("{}: {}", msg.id, msg.message);
    }

    println!("Streaming: {:+<50}", "");
    let mut messages_stream =
        sqlx::query_as::<_, Message>("SELECT id, message FROM messages").fetch(&pool);

    update_message(1, "Hello, Midelt", &pool).await?;

    while let Some(msg) = messages_stream.try_next().await? {
        println!("{}: {}", msg.id, msg.message);
    }

    Ok(())
}

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn update_message(id: i64, message: &str, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? WHERE id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
