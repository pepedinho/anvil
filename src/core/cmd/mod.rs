use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};

use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Build;

pub mod install;
pub mod pack;

pub fn run_build_cmd(build: &Build, project_root: &PathBuf) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} Forging... {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(&build.command)
        .current_dir(project_root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    pb.finish_with_message("Artefact Forged !");

    if !status.success() {
        return Err(anyhow::anyhow!("Build failed with status: {:?}", status));
    }

    Ok(())
}

pub fn run_step(
    cmd: &str,
    dir: Option<&Path>,
    color: &str,
    msg: &str,
    end: &str,
) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::default_spinner()
            .template(&format!("{{spinner:.{color}}} {msg} {{msg}}"))
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );

    pb.enable_steady_tick(Duration::from_millis(100));

    let mut command = std::process::Command::new("sh");

    command.arg("-c").arg(cmd);

    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    if let Some(path) = dir {
        command.current_dir(path);
    }

    let status = command.status()?;

    pb.finish_with_message(end.to_string());

    if !status.success() {
        return Err(anyhow::anyhow!("Command failed with status: {:?}", status));
    }
    Ok(())
}
