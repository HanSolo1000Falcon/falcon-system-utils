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
        println!("{:?}", flags);
    } else {
        if args[1] == "update" {
            if args_len != 2 {
                println!("Can't run update with arguments, continuing execution...");
            }

            return updatehandler::update_program();
        }
    }
    Ok(())
}
