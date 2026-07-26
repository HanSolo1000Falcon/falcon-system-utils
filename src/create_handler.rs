use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use clap::{Subcommand, ValueEnum};

#[derive(Subcommand, Debug)]
pub enum CreateCommand {
    Project {
        name: String,
        lang: LangType
    },
    Header {
        subdir: String,
        name: String,
        lang: LangType
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LangType {
    C,
    Cpp
}

pub fn invoke_create(command: CreateCommand) -> Result<(), Error> {
    match command {
        CreateCommand::Project { name, lang } => create_project(name.as_ref(), &lang),
        CreateCommand::Header { subdir, name, lang }=> create_header(subdir.as_ref(), name.as_ref(), &lang),
    }
}

fn create_project(name: &str, lang: &LangType) -> Result<(), Error> {
    let project_dir: PathBuf = PathBuf::from("./").join(name);
    if project_dir.exists() {
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Directory already exists.",
        ));
    }

    let formatted_name: String = name.replace("-", "_").replace(" ", "_");
    std::fs::create_dir(&project_dir)?;
    std::fs::create_dir(&project_dir.join("include"))?;
    std::fs::create_dir(&project_dir.join("src"))?;

    match lang {
        LangType::C => {
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
                .arg(format!("{}/build", name))
                .arg("-G")
                .arg("Ninja")
                .status()?;
        }
        LangType::Cpp => {
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
                .arg(format!("{}/build", name))
                .arg("-G")
                .arg("Ninja")
                .status()?;
        }
    }

    Command::new("git").arg("init").arg(format!("./{}", name)).status()?;
    Ok(())
}

fn create_header(subdir: &str, name: &str, lang: &LangType) -> Result<(), Error> {
    let header_ext: &str = match lang {
        LangType::C => ".h",
        LangType::Cpp => ".hpp",
    };
    let source_ext: &str = match lang {
        LangType::C => ".c",
        LangType::Cpp => ".cpp",
    };

    let cleaned_path: String = subdir.strip_prefix("/").unwrap_or(&subdir).to_string();
    let header_path: PathBuf = PathBuf::from("./include").join(&cleaned_path);
    let source_path: PathBuf = PathBuf::from("./src").join(&cleaned_path);

    std::fs::create_dir_all(&header_path)?;
    std::fs::create_dir_all(&source_path)?;

    std::fs::write(header_path.join(format!("{}{}", &name, header_ext)), "#pragma once")?;
    std::fs::write(source_path.join(format!("{}{}", &name, source_ext)), "")?;
    Ok(())
}
