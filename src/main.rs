use anyhow::Result;
use clap::Parser;

mod manage_tasks;

/// Simple todo cli app
#[derive(Parser, Debug)]
#[command(author = "Bruno Berrehuel", version, about, long_about = None)]
struct UserCommand {
    #[arg(short, long)]
    command: String,

    /// the task to add
    #[arg(short, long)]
    task_name: Option<String>,

    /// id number of the task (delete)
    #[arg(short, long)]
    id: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = UserCommand::parse();

    match args.command.as_str() {
        "add" => manage_tasks::add_task(args.task_name),
        "list" => manage_tasks::list_tasks().await?,
        "del" => manage_tasks::delete_task(args.id),
        "done" => manage_tasks::task_done(args.id),
        _ => println!("command should be add, del, done, list"),
    };

    Ok(())
}
