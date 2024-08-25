use clap::{Parser, Subcommand};

mod manage_db;
mod manage_tasks;

/// Simple todo cli app
#[derive(Parser)]
#[command(author = "Bruno Berrehuel", version, about, long_about = None)]
struct UserCommand {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { task: String },
    Del { id: u32 },
    Done { id: u32 },
    Todo { id: u32 },
    List { filter: Option<String> },
    Update { id: u32, task: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    manage_db::check_db().await?;
    manage_db::create_table().await?;

    let args = UserCommand::parse();

    match args.cmd {
        Some(Command::Add { task }) => manage_tasks::add_task(task).await?,
        Some(Command::Del { id }) => manage_tasks::delete_task(id).await?,
        Some(Command::Done { id }) => manage_tasks::task_done(id, "done").await?,
        Some(Command::Todo { id }) => manage_tasks::task_done(id, "todo").await?,
        Some(Command::List { filter }) => manage_tasks::list_tasks(filter).await?,
        Some(Command::Update { id, task }) => manage_tasks::update_task(id, task).await?,
        None => manage_tasks::list_tasks(None).await?,
    }

    // NOTE: as_deref to match an Option<String> with &str
    // match args.command.as_deref() {
    //     Some("add") => manage_tasks::add_task(args.task_name).await?,
    //     Some("del") => manage_tasks::delete_task(args.id).await?,
    //     Some("delete") => manage_tasks::delete_task(args.id).await?,
    //     Some("done") => manage_tasks::task_done(args.id, "done").await?,
    //     Some("todo") => manage_tasks::task_done(args.id, "todo").await?,
    //     Some("list") => manage_tasks::list_tasks(args.filter).await?,
    //     Some("update") => manage_tasks::update_task(args.id, args.task_name).await?,
    //     None => manage_tasks::list_tasks(None).await?,
    //     _ => println!("Must use add, del, done, todo, list, update"),
    // };

    Ok(())
}
