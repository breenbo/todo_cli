use crate::manage_db::DB_URL;

pub fn add_task(task_name: Option<String>) {
    if task_name.is_none() {
        println!("You mmust provide a task name");
        return;
    }
    let name = task_name.unwrap_or("".to_string());
    println!("add a task: {}", name);
}

pub async fn list_tasks() -> anyhow::Result<()> {
    println!("list all tasks {}", DB_URL);

    Ok(())
}

pub fn delete_task(task_id: Option<u32>) {
    if task_id.is_none() {
        println!("You must provide the task id to delete");
        return;
    }

    // get the value of an option
    let id = task_id.unwrap_or(0);
    println!("delete task {}", id);
}

pub fn task_done(task_id: Option<u32>) {
    if task_id.is_none() {
        println!("You must provide the task id to delete");
        return;
    }

    let id = task_id.unwrap_or(0);
    println!("task {} done", id);
}
