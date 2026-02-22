use std::process::{Command, Stdio};

use crate::{dotsy_err, error::DotsyError, DotsyResult};

// NOTE: This leaves commands vulnerable to injection if user input is included. This is
// intentional, as the commands are expected to be defined by the user in the config file.
// If we add support for remote profile or configs in the future, we will need to add some
// sort of sanitization or validation to prevent unexpected injection attacks.
pub fn execute_command(command: &str, description: Option<&str>) -> DotsyResult<()> {
    if let Some(desc) = description {
        println!("{}", desc);
    }

    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg(command);

    let mut process = cmd
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|_| DotsyError::FailedToRunCommand)?;

    let status = process.wait().map_err(|_| DotsyError::FailedToRunCommand)?;

    if !status.success() {
        dotsy_err!(DotsyError::FailedToRunCommand);
    }

    Ok(())
}
