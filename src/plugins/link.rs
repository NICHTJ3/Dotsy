use std::{fs, os, path::PathBuf};

use crate::{dotsy_err, error::DotsyError, DotsyResult};

pub fn link_file(from: PathBuf, to: PathBuf) -> DotsyResult<()> {
    if to.is_file() || to.is_dir() {
        dotsy_err!(DotsyError::TODO);
    }

    let from_path_as_string = from.to_str().unwrap().to_string();
    let to_path_as_string = to.to_str().unwrap().to_string();

    if !from.is_file() || !from.is_dir() {
        dotsy_err!(DotsyError::CouldntCreateSymLink {
            from: from_path_as_string,
            to: to_path_as_string,
        });
    }

    if let Err(..) = os::unix::fs::symlink(from, to) {
        dotsy_err!(DotsyError::CouldntCreateSymLink {
            from: from_path_as_string,
            to: to_path_as_string,
        });
    };
    return Ok(());
}

pub fn unlink_file(file: PathBuf) -> DotsyResult<()> {
    let metadata = fs::symlink_metadata(&file).unwrap();
    let file_type = metadata.file_type();

    let is_symlink = file_type.is_symlink();
    if !is_symlink {
        dotsy_err!(DotsyError::TODO);
    }

    if file_type.is_dir() {
        fs::remove_file(&file).unwrap()
    } else {
        fs::remove_file(&file).unwrap()
    }

    return Ok(());
}
