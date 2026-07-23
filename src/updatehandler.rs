use std::env::temp_dir;
use std::fmt::format;
use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub fn update_program() -> Result<(), Error> {
    let project_name: &str = "falcon-system-utils";
    let temp_dir: PathBuf = temp_dir();
    let project_dir: PathBuf = temp_dir.join(project_name);

    let clone_status: ExitStatus = Command::new("git")
        .arg("clone")
        .arg(format!("https://github.com/hansolo1000falcon/{}.git", &project_name))
        .arg(&temp_dir)
        .status()
        .expect("Failed to clone repository");

    if !clone_status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to clone repository",
        ));
    }

    let compile_status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&project_dir.join("Cargo.toml"))
        .status()
        .expect("Failed to compile");

    if !compile_status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to compile",
        ));
    }

    let bin_dir = PathBuf::from(std::env::var("PATH").unwrap()).join(".local/bin");
    std::fs::create_dir_all(&bin_dir).expect("Failed to create directory ~/.local/bin");
    std::fs::rename(project_dir.join("target/release/falcon-system-utils"), bin_dir.join("fsysutils")).expect("Failed to move binary");
    std::fs::remove_dir_all(project_dir).expect("Failed to remove temporary directory");
    Ok(())
}
