pub fn add_task(task_name: Option<String>) {
    if task_name.is_none() {
        println!("You mmust provide a task name");
        return;
    }
    let name = task_name.unwrap_or("".to_string());
    println!("add a task: {}", name);
}

pub fn list_tasks() {
    println!("list all tasks")
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

pub fn task_done(id: Option<u32>) {
    println!("task done");
}
