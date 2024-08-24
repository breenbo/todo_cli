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
    task_id: Option<u32>,
}

fn main() {
    let args = UserCommand::parse();

    match args.command.as_str() {
        "add" => manage_tasks::add_task(args.task_name),
        "list" => manage_tasks::list_tasks(),
        "del" => manage_tasks::delete_task(args.task_id),
        "done" => manage_tasks::task_done(args.task_id),
        _ => println!("command should be add, del, done, list"),
    }
}
