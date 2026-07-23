mod updatehandler;

use std::collections::HashMap;
use std::io::Error;

fn main() -> Result<(), Error> {
    if std::env::consts::OS != "linux" {
        println!("Linux only program, expect broken behaviour on other platforms.");
    }

    let acceptable_flags: HashMap<char, &str> =
        HashMap::from([('h', "--help"), ('v', "--version")]);
    let acceptable_flags_vals: Vec<&&str> = acceptable_flags.values().collect();

    let args: Vec<String> = std::env::args().collect();
    let args_len: usize = args.len();
    if args_len < 2 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid usage. Run with the --help|-h flag for usage instructions.",
        ));
    }

    let contains_flag: bool = args.iter().any(|arg| arg.starts_with('-'));
    if contains_flag {
        let mut flags: Vec<&str> = Vec::new();
        for (idx, arg) in args.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            if !arg.starts_with('-') {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid usage. Can't mix flags and arguments.",
                ));
            }

            let is_expanded_flag: bool = arg.starts_with("--");
            if is_expanded_flag && acceptable_flags_vals.contains(&&arg.as_ref()) {
                flags.push(arg);
            } else if is_expanded_flag {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid usage. Unknown flag.",
                ));
            } else {
                let arg_chars: Vec<char> = arg.chars().collect();
                for i in 1..arg.len() {
                    let flag: char = arg_chars[i];
                    if acceptable_flags.contains_key(&flag) {
                        flags.push(acceptable_flags.get(&flag).unwrap());
                    } else {
                        return Err(Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid usage. Unknown flag.",
                        ));
                    }
                }
            }
        }

        for flag in flags {
            match flag {
                "--help" => {
                    println!(
                        r"
fsysutils help wizard

Commands:
    update - Update fsysutils to the latest version
    dotfiles:
        pull|fetch|install - Gets the latest dotfiles from the specified .config repository (default: https://github.com/hansolo1000falcon/.config.git)
        push - Pushes the specified .config repository (default: https://github.com/hansolo1000falcon/.config.git)
        set-remote <remote-url> - Sets the remote repository for the dotfiles
    create:
        project <project-name> <c|cpp> - Creates a new project with the specified name and language
        header <header-name> <subdir> <c|cpp> - Creates a new header file with the specified name and language in the specified subdirectory of include/

Flags:
    -h, --help - Prints this help message
    -v, --version - Prints the version of fsysutils"
                    )
                }
                "--version" => println!("fsysutils version: {}", env!("CARGO_PKG_VERSION")),
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid usage. Unknown flag.",
                    ));
                }
            }
        }
    } else {
        return match args[1].as_ref() {
            "update" => updatehandler::update_program(),
            _ => Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid usage. Unknown command.")),
        }
    }
    Ok(())
}
