use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    remote: String,
    add: Vec<String>,
}

fn fetch_config() -> Result<Config, Error> {
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config/fsysutils");
    let config_path: PathBuf = config_dir.join("dotfiles-config.toml");
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    if config_path.exists() {
        let contents: String = std::fs::read_to_string(&config_path).expect("Failed to read config file");
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

    let mut config: Config = fetch_config().expect("Failed to fetch config");

    let command: &str = &args[2];
    match command {
        "pull" | "fetch" | "install" => fetch_dotfiles(&config).expect("Failed to fetch dotfiles"),
        "push" => push_dotfiles(&config).expect("Failed to push dotfiles"),
        "add" => add_dotfiles(&mut config, args).expect("Failed to add dotfiles"),
        "set-remote" => set_remote_dotfiles(&mut config, args).expect("Failed to set remote dotfiles"),
        _ => return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid usage. Unknown command.")),
    }

    std::fs::write(PathBuf::from(env!("HOME")).join(".config/fsysutils/dotfiles-config.toml"), toml::to_string_pretty(&config).expect("Failed to make config into toml")).expect("Failed to write config file");
    Ok(())
}

fn fetch_dotfiles(config: &Config) -> Result<(), Error> {
    let config_directory: PathBuf = PathBuf::from(env!("HOME")).join(".config");

    if !config_directory.join(".git").exists() {
        Command::new("git").arg("-C").arg(&config_directory).arg("init").status().expect("Failed to initialize git repository");
        Command::new("git").arg("-C").arg(&config_directory).arg("remote").arg("add").arg("origin").arg(&config.remote).status().expect("Failed to add remote repository");
        Command::new("git").arg("-C").arg(&config_directory).arg("fetch").status().expect("Failed to fetch remote repository");
        Command::new("git").arg("-C").arg(&config_directory).arg("checkout").arg("-b").arg("main").arg("--track").arg("origin/main").status().expect("Failed to checkout main branch");
    }

    Command::new("git").arg("-C").arg(&config_directory).arg("fetch").arg("origin").status().expect("Failed to pull remote repository");
    Command::new("git").arg("-C").arg(&config_directory).arg("reset").arg("--hard").arg("origin/main").status().expect("Failed to reset repository");
    Ok(())
}

fn push_dotfiles(config: &Config) -> Result<(), Error> {
    let config_dir: PathBuf = PathBuf::from(env!("HOME")).join(".config");
    Command::new("git").arg("-C").arg(&config_dir).arg("add").args(&config.add).status().expect("Failed to add all files");
    Command::new("git").arg("-C").arg(&config_dir).arg("commit").arg("-m").arg("Update dotfiles").status().expect("Failed to commit changes");
    Command::new("git").arg("-C").arg(&config_dir).arg("push").arg("origin").arg("main").status().expect("Failed to push changes");
    Ok(())
}

fn add_dotfiles(config: &mut Config, args: Vec<String>) -> Result<(), Error> {
    if args.len() < 4 {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid usage. Run with the --help|-h flag for usage instructions."));
    }

    for i in 3..args.len() {
        config.add.push(args[i].clone());
    }
    Ok(())
}

fn set_remote_dotfiles(config: &mut Config, args: Vec<String>) -> Result<(), Error> {
    if args.len() != 4 {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid usage. Run with the --help|-h flag for usage instructions."));
    }

    config.remote = args[3].clone();
    Ok(())
}