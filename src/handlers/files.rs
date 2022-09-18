use std::{fs, path::PathBuf};

use crate::{dotsy_log_warning, DotsyResult};

// TODO: Should all of these handlers have a do and undo? default implementation for undo should
// just log "Could not undo `<module name>` you may want to look at this yourself!!"
// TODO: Might need more work
pub fn create_dir(dir: PathBuf) -> DotsyResult<()> {
    println!("Attempting to create directory: {}", &dir.to_string_lossy());
    if dir.exists() && (dir.is_file() || dir.is_dir()) {
        dotsy_log_warning!("Directory already exists skipping creation...");
        return Ok(());
    }

    fs::create_dir_all(dir).unwrap();

    Ok(())
}
