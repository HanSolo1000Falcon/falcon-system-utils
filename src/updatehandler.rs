use std::env::temp_dir;
use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub fn update_program() -> Result<(), Error> {
    let project_name: &str = "falcon-system-utils";
    let project_dir: PathBuf = temp_dir().join(project_name);

    let clone_status: ExitStatus = Command::new("git")
        .arg("clone")
        .arg(format!(
            "https://github.com/hansolo1000falcon/{}.git",
            &project_name
        ))
        .arg(&project_dir)
        .status()
        .expect("Failed to clone repository");

    if !clone_status.success() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to clone repository",
        ));
    }

    let compile_status: ExitStatus = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&project_dir.join("Cargo.toml"))
        .status()
        .expect("Failed to compile");

    if !compile_status.success() {
        return Err(Error::new(std::io::ErrorKind::Other, "Failed to compile"));
    }

    let bin_dir: PathBuf = PathBuf::from(env!("HOME")).join(".local/bin");
    let bin_path: PathBuf = project_dir.join(format!("target/release/{}", project_name));
    std::fs::copy(bin_path, bin_dir.join(project_name)).expect("Failed to copy binary");
    std::fs::rename(bin_dir.join(project_name), bin_dir.join("fsysutils"))
        .expect("Failed to rename binary");
    std::fs::remove_dir_all(project_dir).expect("Failed to remove temporary directory");
    Ok(())
}
