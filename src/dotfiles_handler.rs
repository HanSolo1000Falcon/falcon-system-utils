use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::fs::DirEntry;
use std::io::Error;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    remote: String,
}

#[derive(Subcommand, Debug)]
pub enum DotfilesCommand {
    Pull,
    Fetch,
    Install,
    Push,
    Add { add: Vec<String> },
    Remove { remove: Vec<String> },
    SetRemote { remote: String },
}

fn fetch_config() -> Result<Config, Error> {
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config/fsysutils");
    let config_path: PathBuf = config_dir.join("dotfiles-config.toml");
    std::fs::create_dir_all(&config_dir)?;
    if config_path.exists() {
        let contents: String = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents).unwrap_or_else(|_| Config {
            remote: String::from("https://github.com/hansolo1000falcon/.config.git"),
        });
        Ok(config)
    } else {
        Ok(Config {
            remote: String::from("https://github.com/hansolo1000falcon/.config.git"),
        })
    }
}

pub fn invoke_dotfiles(command: DotfilesCommand) -> Result<(), Box<dyn std::error::Error>> {
    let mut config: Config = fetch_config()?;

    match command {
        DotfilesCommand::Pull | DotfilesCommand::Fetch | DotfilesCommand::Install => {
            fetch_dotfiles(&config)?
        }
        DotfilesCommand::Push => push_dotfiles()?,
        DotfilesCommand::Add { add } => add_dotfiles(&add)?,
        DotfilesCommand::Remove { remove } => remove_dotfiles(&remove)?,
        DotfilesCommand::SetRemote { remote } => set_remote_dotfiles(&mut config, &remote)?,
    }

    std::fs::write(
        PathBuf::from(env!("HOME")).join(".config/fsysutils/dotfiles-config.toml"),
        toml::to_string_pretty(&config).unwrap(),
    )?;
    Ok(())
}

fn fetch_dotfiles(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir: PathBuf = PathBuf::from(env!("HOME")).join(".local/share/.config-repo/");
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config");

    if !repo_dir.exists() {
        Command::new("git")
            .arg("clone")
            .arg(&config.remote)
            .arg(&repo_dir)
            .status()?;
    } else {
        Command::new("git")
            .arg("-C")
            .arg(&repo_dir)
            .arg("pull")
            .status()?;
    }

    for file in std::fs::read_dir(&repo_dir)? {
        let file: DirEntry = file?;
        let dst: PathBuf = file.path();

        if file.file_name() == ".git" {
            continue;
        }

        let src: PathBuf = config_dir.join(file.file_name());
        if src.exists() || src.is_symlink() {
            if src.is_dir() && !src.is_symlink() {
                std::fs::remove_dir_all(&src)?;
            } else {
                std::fs::remove_file(&src)?;
            }
        }
        symlink(&dst, &src)?;
    }

    Ok(())
}

fn push_dotfiles() -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir: PathBuf = PathBuf::from(env!("HOME")).join(".local/share/.config-repo/");
    Command::new("git")
        .arg("-C")
        .arg(&repo_dir)
        .arg("add")
        .arg(".")
        .status()?;
    Command::new("git")
        .arg("-C")
        .arg(&repo_dir)
        .arg("commit")
        .arg("-m")
        .arg("Update dotfiles with fsysutils")
        .status()?;
    Command::new("git")
        .arg("-C")
        .arg(&repo_dir)
        .arg("push")
        .status()?;
    Ok(())
}

fn add_dotfiles(add: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config");
    let repo_dir: PathBuf = PathBuf::from(env!("HOME")).join(".local/share/.config-repo/");

    for file in add {
        let src: PathBuf = config_dir.join(file);
        let dst: PathBuf = repo_dir.join(file);

        if !src.exists() {
            eprintln!("Skipping {file}, doesn't exist in ~/.config");
            continue;
        }
        if src.is_symlink() {
            eprintln!("Skipping {file}, already tracked (already a symlink)");
            continue;
        }

        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::rename(&src, &dst)?;
        symlink(&dst, &src)?;

        println!("Tracked {file}");
    }

    Ok(())
}

fn remove_dotfiles(remove: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config");
    let repo_dir: PathBuf = PathBuf::from(env!("HOME")).join(".local/share/.config-repo/");

    for file in remove {
        let link: PathBuf = config_dir.join(file);
        let stored: PathBuf = repo_dir.join(file);

        if !link.is_symlink() {
            eprintln!("Skipping {file}, not a tracked symlink");
            continue;
        }
        if !stored.exists() {
            eprintln!("Skipping {file}, missing from repo, can't restore");
            continue;
        }

        std::fs::remove_file(&link)?;
        std::fs::rename(&stored, &link)?;

        println!("Untracked {file}");
    }

    Ok(())
}

fn set_remote_dotfiles(config: &mut Config, remote: &String) -> Result<(), Box<dyn std::error::Error>> {
    config.remote = remote.clone();
    Ok(())
}
