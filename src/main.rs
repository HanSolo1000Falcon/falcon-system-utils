mod create_handler;
mod dotfiles_handler;
mod exchange_handler;
mod update_handler;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Update,
    Create {
        #[command(subcommand)]
        subcommand: create_handler::CreateCommand,
    },
    Dotfiles {
        #[command(subcommand)]
        subcommand: dotfiles_handler::DotfilesCommand,
    },
    Exchange {
        #[clap(short, long)]
        from: Option<String>,
        #[clap(short, long)]
        to: Option<String>,
        #[clap(short, long)]
        amount: Option<f64>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::consts::OS != "linux" {
        println!("Linux only program, expect broken behaviour on other platforms.");
    }

    let args: Args = Args::parse();
    match args.command {
        Command::Update => update_handler::update_program(),
        Command::Create { subcommand } => create_handler::invoke_create(subcommand),
        Command::Dotfiles { subcommand } => dotfiles_handler::invoke_dotfiles(subcommand),
        Command::Exchange { from, to, amount } => {
            exchange_handler::invoke_exchange(from, to, amount)
        }
    }
}
