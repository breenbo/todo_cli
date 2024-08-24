use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub const DB_URL: &str = "sqlite://todos.db";

pub async fn check_db() -> anyhow::Result<()> {
    println!("check db and table");
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);

        Sqlite::create_database(DB_URL).await?;
    } else {
        println!("Db already exists")
    }

    Ok(())
}

pub async fn create_table() -> anyhow::Result<()> {
    println!("create table tasks");
    let db = SqlitePool::connect(DB_URL).await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, task VARCHAR(255) NOT NULL);")
        .execute(&db)
        .await?;

    Ok(())
}
