use std::process::{Command, Stdio};

use crate::{dotsy_err, error::DotsyError, DotsyResult};

pub fn run_script(script: &str) -> DotsyResult<()> {
    let mut command = Command::new(script);
    let mut command = {
        let this = command.stdout(Stdio::inherit()).spawn();
        match this {
            Ok(t) => t,
            Err(..) => dotsy_err!(DotsyError::FailedToRunCommand),
        }
    };

    match command.wait() {
        Ok(t) => t,
        Err(..) => dotsy_err!(DotsyError::FailedToRunCommand),
    };

    Ok(())
}
