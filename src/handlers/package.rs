use std::process::{Command, Stdio};

use crate::{dotsy_err, dotsy_log_error, error::DotsyError, DotsyResult};

fn execute_package_command(package: &str, command_template: &str, operation: &str) -> DotsyResult<()> {
    let command = command_template.replace("{}", package);
    println!("Attempting to {} package: {}", operation, package);
    
    // Use shell execution to properly handle complex commands with quotes, pipes, etc.
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd.arg(&command);

    let mut process = {
        let this = cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn();
        match this {
            Ok(t) => t,
            Err(e) => {
                dotsy_log_error!("Failed to spawn command: {}", e);
                return dotsy_err!(DotsyError::FailedToRunCommand);
            }
        }
    };

    match process.wait() {
        Ok(status) => {
            if !status.success() {
                dotsy_log_error!("Package {} failed with status: {}", operation, status);
                return dotsy_err!(DotsyError::FailedToRunCommand);
            }
        }
        Err(e) => {
            dotsy_log_error!("Failed to wait for command: {}", e);
            return dotsy_err!(DotsyError::FailedToRunCommand);
        }
    };

    Ok(())
}

pub fn install_package(package: &str, install_command: &str) -> DotsyResult<()> {
    execute_package_command(package, install_command, "install")
}

pub fn uninstall_package(package: &str, uninstall_command: &str) -> DotsyResult<()> {
    execute_package_command(package, uninstall_command, "uninstall")
}
