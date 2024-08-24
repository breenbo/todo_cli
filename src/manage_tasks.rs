use sqlx::{FromRow, SqlitePool};

use crate::manage_db::DB_URL;

pub async fn add_task(task_name: Option<String>) -> anyhow::Result<()> {
    if task_name.is_none() {
        println!("You mmust provide a task name");
        return Ok(());
    }
    let name = task_name.unwrap_or("".to_string());
    println!("add a task: {}", name);

    let db = SqlitePool::connect(DB_URL).await?;

    sqlx::query("INSERT INTO todos (task) VALUES (?)")
        .bind(name)
        .execute(&db)
        .await?;

    db.close().await;

    Ok(())
}

// NOTE: create a struc to get stuctured data from the query
#[derive(Clone, FromRow, Debug)]
struct Task {
    id: u32,
    task: String,
    done: bool
}

pub async fn list_tasks(filter: Option<String>) -> anyhow::Result<()> {
    let db = SqlitePool::connect(DB_URL).await?;

    // NOTE: create query depending on the filter
    let query = match filter.as_deref() {
        Some("done") => "SELECT id, task, done FROM todos WHERE done = TRUE",
        Some("todo") => "SELECT id, task, done FROM todos WHERE done = FALSE",
        None => "SELECT id, task, done FROM todos",
        Some(_) => {
            println!("filter is 'done' / 'todo'");
            ""
        }
    };

    if query.is_empty() {
        return Ok(());
    }

    let tasks = 
    // NOTE: use query_as to get structured data from the query
            sqlx::query_as::<_, Task>(query)
                .fetch_all(&db)
                .await?;

    if tasks.is_empty() {
        println!("No task to display");
        return Ok(());
    }

    for task in tasks {
        let done = match task.done {
            true => "done",
            false => "todo"
        };

        println!("{}: {} {}", task.id, task.task, done)
    }

    db.close().await;

    Ok(())
}

pub async fn delete_task(task_id: Option<u32>) -> anyhow::Result<()> {
    if task_id.is_none() {
        println!("You must provide the task id to delete");
        return Ok(());
    }

    let db = SqlitePool::connect(DB_URL).await?;

    let id = task_id.unwrap_or(0);
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&db)
        .await?;

    // get the value of an option
    let id = task_id.unwrap_or(0);
    println!("delete task {}", id);

    db.close().await;
    
    Ok(())
}

pub async fn task_done(task_id: Option<u32>) -> anyhow::Result<()> {
    if task_id.is_none() {
        println!("You must provide the task id to delete");
        return Ok(());
    }

    let db = SqlitePool::connect(DB_URL).await?;
    let id = task_id.unwrap_or(0);

    sqlx::query("UPDATE todos SET done = TRUE WHERE id = ?")
        .bind(id)
        .execute(&db)
        .await?;

    db.close().await;

    Ok(())
}

pub async fn update_task(task_id: Option<u32>, task_name: Option<String>) -> anyhow::Result<()> {
    if task_id.is_none() {
        println!("must provide task id to update");
        return Ok(());
    }

    if task_name.is_none() {
        println!("must provide new task to update");
        return Ok(())
    }

    let db = SqlitePool::connect(DB_URL).await?;
    let id = task_id.unwrap_or(0);
    let name = task_name.unwrap_or("".to_string());

    sqlx::query("UPDATE todos SET task = $1 WHERE id = $2")
        .bind(name)
        .bind(id)
        .execute(&db)
        .await?;
    
    db.close().await;

    Ok(())

}
