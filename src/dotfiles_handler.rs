use serde::{Deserialize, Serialize};
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use clap::Subcommand;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    remote: String,
    add: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum DotfilesCommand {
    Pull,
    Fetch,
    Install,
    Push,
    Add {
        add: Vec<String>,
    },
    SetRemote {
        remote: String,
    },
}

fn fetch_config() -> Result<Config, Error> {
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config/fsysutils");
    let config_path: PathBuf = config_dir.join("dotfiles-config.toml");
    std::fs::create_dir_all(&config_dir)?;
    if config_path.exists() {
        let contents: String = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents).unwrap_or_else(|_| Config {
            remote: String::from("https://github.com/hansolo1000falcon/.config.git"),
            add: Vec::new(),
        });
        Ok(config)
    } else {
        Ok(Config {
            remote: String::from("https://github.com/hansolo1000falcon/.config.git"),
            add: Vec::new(),
        })
    }
}

pub fn invoke_dotfiles(command: DotfilesCommand) -> Result<(), Error> {
    let mut config: Config = fetch_config()?;

    match command {
        DotfilesCommand::Pull | DotfilesCommand::Fetch | DotfilesCommand::Install => fetch_dotfiles(&config)?,
        DotfilesCommand::Push => push_dotfiles(&config)?,
        DotfilesCommand::Add { add } => add_dotfiles(&mut config, &add)?,
        DotfilesCommand::SetRemote { remote } => set_remote_dotfiles(&mut config, &remote)?,
    }

    std::fs::write(
        PathBuf::from(env!("HOME")).join(".config/fsysutils/dotfiles-config.toml"),
        toml::to_string_pretty(&config).unwrap(),
    )?;
    Ok(())
}

fn fetch_dotfiles(config: &Config) -> Result<(), Error> {
    let config_directory: PathBuf = PathBuf::from(env!("HOME")).join(".config");

    if !config_directory.join(".git").exists() {
        Command::new("git")
            .arg("-C")
            .arg(&config_directory)
            .arg("init")
            .status()?;
        Command::new("git")
            .arg("-C")
            .arg(&config_directory)
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(&config.remote)
            .status()?;
        Command::new("git")
            .arg("-C")
            .arg(&config_directory)
            .arg("fetch")
            .status()?;
        Command::new("git")
            .arg("-C")
            .arg(&config_directory)
            .arg("checkout")
            .arg("-b")
            .arg("main")
            .arg("--track")
            .arg("origin/main")
            .status()?;
    }

    Command::new("git")
        .arg("-C")
        .arg(&config_directory)
        .arg("fetch")
        .arg("origin")
        .status()?;
    Command::new("git")
        .arg("-C")
        .arg(&config_directory)
        .arg("reset")
        .arg("--hard")
        .arg("origin/main")
        .status()?;
    Ok(())
}

fn push_dotfiles(config: &Config) -> Result<(), Error> {
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config");
    Command::new("git")
        .arg("-C")
        .arg(&config_dir)
        .arg("add")
        .args(&config.add)
        .status()?;
    Command::new("git")
        .arg("-C")
        .arg(&config_dir)
        .arg("commit")
        .arg("-m")
        .arg("Update dotfiles")
        .status()?;
    Command::new("git")
        .arg("-C")
        .arg(&config_dir)
        .arg("push")
        .arg("origin")
        .arg("main")
        .status()?;
    Ok(())
}

fn add_dotfiles(config: &mut Config, add: &Vec<String>) -> Result<(), Error> {
    for to_add in add {
        config.add.push(to_add.clone());
    }
    Ok(())
}

fn set_remote_dotfiles(config: &mut Config, remote: &String) -> Result<(), Error> {
    config.remote = remote.clone();
    Ok(())
}
