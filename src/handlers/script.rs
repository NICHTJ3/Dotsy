use std::process::{Command, Stdio};

use crate::{dotsy_err, error::DotsyError, DotsyResult};

pub fn run_script(script: &str) -> DotsyResult<()> {
    println!("Attempting to run script: {}", script);
    // This should probably be made cross platform
    let mut command = Command::new("bash");
    command.arg("-c");
    command.arg(script);

    let mut command = {
        let this = command.stdout(Stdio::inherit()).spawn();
        match this {
            Ok(t) => t,
            Err(..) => {
                return dotsy_err!(DotsyError::FailedToRunCommand)
            }
        }
    };

    match command.wait() {
        Ok(t) => t,
        Err(..) => {
            return dotsy_err!(DotsyError::FailedToRunCommand)
        }
    };

    Ok(())
}
