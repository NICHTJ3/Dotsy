use std::process::{Command, Stdio};

use crate::{dotsy_err, dotsy_log_error, error::DotsyError, DotsyResult};

pub fn install_package(package: &str, install_command: &str) -> DotsyResult<()> {
    let command = install_command.replace("{}", package);
    println!("Attempting to install package: {}", package);

    // Split the command into parts for proper execution
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        dotsy_err!(DotsyError::FailedToRunCommand {
            details: "Package install command is empty".to_string()
        });
    }

    let mut cmd = Command::new(parts[0]);
    for arg in &parts[1..] {
        cmd.arg(arg);
    }

    let mut process = {
        let this = cmd.stdout(Stdio::inherit()).spawn();
        match this {
            Ok(t) => t,
            Err(e) => {
                dotsy_log_error!("Failed to spawn command: {}", e);
                dotsy_err!(DotsyError::FailedToRunCommand {
                    details: format!("Failed to spawn package install command: {}", e)
                });
            }
        }
    };

    match process.wait() {
        Ok(status) => {
            if !status.success() {
                dotsy_log_error!("Package installation failed with status: {}", status);
                dotsy_err!(DotsyError::FailedToRunCommand {
                    details: format!("Package installation failed with status: {}", status)
                });
            }
        }
        Err(e) => {
            dotsy_log_error!("Failed to wait for command: {}", e);
            dotsy_err!(DotsyError::FailedToRunCommand {
                details: format!("Failed to wait for package install command: {}", e)
            });
        }
    };

    Ok(())
}

pub fn uninstall_package(package: &str, uninstall_command: &str) -> DotsyResult<()> {
    let command = uninstall_command.replace("{}", package);
    println!("Attempting to uninstall package: {}", package);

    // Split the command into parts for proper execution
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        dotsy_err!(DotsyError::FailedToRunCommand {
            details: "Package uninstall command is empty".to_string()
        });
    }

    let mut cmd = Command::new(parts[0]);
    for arg in &parts[1..] {
        cmd.arg(arg);
    }

    let mut process = {
        let this = cmd.stdout(Stdio::inherit()).spawn();
        match this {
            Ok(t) => t,
            Err(e) => {
                dotsy_log_error!("Failed to spawn command: {}", e);
                dotsy_err!(DotsyError::FailedToRunCommand {
                    details: format!("Failed to spawn package uninstall command: {}", e)
                });
            }
        }
    };

    match process.wait() {
        Ok(status) => {
            if !status.success() {
                dotsy_log_error!("Package uninstallation failed with status: {}", status);
                dotsy_err!(DotsyError::FailedToRunCommand {
                    details: format!("Package uninstallation failed with status: {}", status)
                });
            }
        }
        Err(e) => {
            dotsy_log_error!("Failed to wait for command: {}", e);
            dotsy_err!(DotsyError::FailedToRunCommand {
                details: format!("Failed to wait for package uninstall command: {}", e)
            });
        }
    };

    Ok(())
}
