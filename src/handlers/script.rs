use crate::{shell, DotsyResult};

pub fn run_script(script: &str) -> DotsyResult<()> {
    shell::execute_command(
        script,
        Some(&format!("Attempting to run script: {}", script)),
    )
}
