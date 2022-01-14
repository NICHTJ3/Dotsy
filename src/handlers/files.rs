use std::{fs, path::PathBuf};

use crate::{dotsy_warn, DotsyResult};

// TODO: Should all of these handlers have a do and undo? default implementation for undo should
// just log "Could not undo `<module name>` you may want to look at this yourself!!"
// TODO: Might need more work
pub fn create_dir(dir: PathBuf) -> DotsyResult<()> {
    if dir.exists() && (dir.is_file() || dir.is_dir()) {
        dotsy_warn!(
            "Directory `{dir}` already exists skipping creation...",
            dir = dir.clone().to_string_lossy()
        );
    }

    fs::create_dir_all(dir).unwrap();

    Ok(())
}
