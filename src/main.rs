mod dotfiles_handler;
mod update_handler;
mod create_handler;

use std::io::Error;
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
}

fn main() -> Result<(), Error> {
    if std::env::consts::OS != "linux" {
        println!("Linux only program, expect broken behaviour on other platforms.");
    }

    let args: Args = Args::parse();
    match args.command {
        Command::Update => update_handler::update_program(),
        Command::Create { subcommand } => create_handler::invoke_create(subcommand),
        Command::Dotfiles { subcommand } => dotfiles_handler::invoke_dotfiles(subcommand),
    }
}
