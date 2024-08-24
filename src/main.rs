use clap::Parser;

mod manage_db;
mod manage_tasks;

/// Simple todo cli app
#[derive(Parser, Debug)]
#[command(author = "Bruno Berrehuel", version, about, long_about = None)]
struct UserCommand {
    #[arg(short, long)]
    command: Option<String>,

    /// the task to add
    #[arg(short, long)]
    task_name: Option<String>,

    /// id number of the task (delete)
    #[arg(short, long)]
    id: Option<u32>,

    /// id number of the task (delete)
    #[arg(short, long)]
    filter: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    manage_db::check_db().await?;
    manage_db::create_table().await?;

    let args = UserCommand::parse();

    // NOTE: as_deref to match an Option<String> with &str
    match args.command.as_deref() {
        Some("add") => manage_tasks::add_task(args.task_name).await?,
        Some("del") => manage_tasks::delete_task(args.id).await?,
        Some("done") => manage_tasks::task_done(args.id).await?,
        Some("list") => manage_tasks::list_tasks(args.filter).await?,
        Some("update") => manage_tasks::update_task(args.id, args.task_name).await?,
        None => manage_tasks::list_tasks(None).await?,
        _ => println!("Must use add, del, done or nothing"),
    };

    Ok(())
}
