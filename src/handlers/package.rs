use crate::{shell, DotsyResult};

fn execute_package_command(
    package: &str,
    command_template: &str,
    operation: &str,
) -> DotsyResult<()> {
    let command = command_template.replace("{}", package);
    shell::execute_command(
        &command,
        Some(&format!("Attempting to {} package: {}", operation, package)),
    )
}

pub fn install_package(package: &str, install_command: &str) -> DotsyResult<()> {
    execute_package_command(package, install_command, "install")
}

pub fn uninstall_package(package: &str, uninstall_command: &str) -> DotsyResult<()> {
    execute_package_command(package, uninstall_command, "uninstall")
}
