use serde::{Deserialize, Serialize};
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    remote: String,
    add: Vec<String>,
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

pub fn invoke_dotfiles(args: Vec<String>) -> Result<(), Error> {
    if args.len() < 3 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid usage. Run with the --help|-h flag for usage instructions.",
        ));
    }

    let mut config: Config = fetch_config()?;

    match args[2].as_ref() {
        "pull" | "fetch" | "install" => fetch_dotfiles(&config)?,
        "push" => push_dotfiles(&config)?,
        "add" => add_dotfiles(&mut config, args)?,
        "set-remote" => set_remote_dotfiles(&mut config, args)?,
        _ => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid usage. Unknown command.",
            ));
        }
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

fn add_dotfiles(config: &mut Config, args: Vec<String>) -> Result<(), Error> {
    if args.len() < 4 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid usage. Run with the --help|-h flag for usage instructions.",
        ));
    }

    for i in 3..args.len() {
        config.add.push(args[i].clone());
    }
    Ok(())
}

fn set_remote_dotfiles(config: &mut Config, args: Vec<String>) -> Result<(), Error> {
    if args.len() != 4 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid usage. Run with the --help|-h flag for usage instructions.",
        ));
    }

    config.remote = args[3].clone();
    Ok(())
}
