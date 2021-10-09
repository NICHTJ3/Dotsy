use std::{fs, path::PathBuf};

use crate::{dotsy_err, error::DotsyError, DotsyResult};

// TODO: Might need more work
pub fn create_dir(dir: PathBuf) -> DotsyResult<()> {
    if dir.is_file() || dir.is_dir() {
        dotsy_err!(DotsyError::TODO);
    }

    fs::create_dir(dir).unwrap();

    Ok(())
}
