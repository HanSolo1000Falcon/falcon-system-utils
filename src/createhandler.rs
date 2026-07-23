use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

pub fn invoke_create(args: Vec<String>) -> Result<(), Error> {
    if args.len() < 3 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid usage. Run with the --help|-h flag for usage instructions.",
        ));
    }

    match args[2].as_ref() {
        "project" => create_project(args),
        "header" => create_header(args),
        _ => Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid usage. Unknown command.",
        )),
    }
}

fn create_project(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 5 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid usage. Run with the --help|-h flag for usage instructions.",
        ));
    }

    let project_dir: PathBuf = PathBuf::from("./").join(args[3].clone());
    if project_dir.exists() {
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Directory already exists.",
        ));
    }

    let formatted_name: String = args[3].clone().replace("-", "_").replace(" ", "_");
    std::fs::create_dir(&project_dir)?;
    std::fs::create_dir(&project_dir.join("include"))?;
    std::fs::create_dir(&project_dir.join("src"))?;

    match args[4].as_ref() {
        "c" => {
            std::fs::write(
                project_dir.join("src/main.c"),
                "int main(void) { return 0; }",
            )?;
            std::fs::write(
                project_dir.join("CMakeLists.txt"),
                format!(
                    r"cmake_minimum_required(VERSION 4.1)

project({} LANGUAGES C)

set(CMAKE_C_STANDARD 23)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_SCAN_FOR_MODULES OFF)

add_executable({} src/main.c)
target_include_directories({} PRIVATE include)
            ",
                    &formatted_name, &formatted_name, &formatted_name
                ),
            )?;
            std::fs::write(project_dir.join(".gitignore"), "build/")?;
            Command::new("cmake")
                .arg("-S")
                .arg(&project_dir)
                .arg("-B")
                .arg(format!("{}/build", args[3]))
                .arg("-G")
                .arg("Ninja")
                .status()?;
        }
        "cpp" => {
            std::fs::write(project_dir.join("src/main.cpp"), "int main() { return 0; }")?;
            std::fs::write(
                project_dir.join("CMakeLists.txt"),
                format!(
                    r"cmake_minimum_required(VERSION 3.31.10)

project({} LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_SCAN_FOR_MODULES OFF)

add_executable({} src/main.cpp)
target_include_directories({} PRIVATE include)
            ",
                    &formatted_name, &formatted_name, &formatted_name
                ),
            )?;
            std::fs::write(project_dir.join(".gitignore"), "build/")?;
            Command::new("cmake")
                .arg("-S")
                .arg(&project_dir)
                .arg("-B")
                .arg(format!("{}/build", args[3]))
                .arg("-G")
                .arg("Ninja")
                .status()?;
        }
        _ => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid language specified.",
            ));
        }
    }

    Ok(())
}

fn create_header(args: Vec<String>) -> Result<(), Error> {}
