//
// https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
//
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub const DB_URL: &str = "sqlite://todos.db";

pub async fn check_db() -> anyhow::Result<()> {
    println!("check db and table");
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        Sqlite::create_database(DB_URL).await?;
    }

    Ok(())
}

pub async fn create_table() -> anyhow::Result<()> {
    let db = SqlitePool::connect(DB_URL).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
            task TEXT NOT NULL,
            done BOOLEAN DEFAULT FALSE
     );",
    )
    .execute(&db)
    .await?;

    db.close().await;

    Ok(())
}
