use std::{fs, os, path::PathBuf};

use crate::{dotsy_err, error::DotsyError, DotsyResult};

fn link(from: &PathBuf, to: &PathBuf) -> DotsyResult<()> {
    let from_path_as_string = from.to_str().unwrap().to_string();
    let to_path_as_string = to.to_str().unwrap().to_string();

    let to_dir = to.parent().unwrap();
    if !to_dir.exists() {
        fs::create_dir_all(to_dir).unwrap();
    };

    if let Err(..) = os::unix::fs::symlink(to, from) {
        dotsy_err!(DotsyError::CouldntCreateSymLink {
            from: from_path_as_string,
            to: to_path_as_string
        });
    }

    return Ok(());
}

// TODO: Rethink how this is passed should this actually receive the whole link object?
// NOTE: `_glob` will eventually be used but not quite yet
pub fn link_file(from: PathBuf, to: PathBuf, _glob: bool) -> DotsyResult<()> {
    let from_path_as_string = from.to_str().unwrap().to_string();
    let to_path_as_string = to.to_str().unwrap().to_string();

    if to.exists() && (to.is_file() || to.is_dir()) || !from.is_file() || !from.is_dir() {
        dotsy_err!(DotsyError::CouldntCreateSymLink {
            from: from_path_as_string,
            to: to_path_as_string,
        });
    }

    link(&from, &to)
}

pub fn unlink_file(file: &PathBuf) -> DotsyResult<()> {
    let metadata = fs::symlink_metadata(&file).unwrap();
    let file_type = metadata.file_type();

    let is_symlink = file_type.is_symlink();
    if !is_symlink {
        dotsy_err!(DotsyError::Unlink {
            link: file.to_str().unwrap().to_string()
        });
    }

    if file_type.is_dir() {
        fs::remove_file(&file).unwrap()
    } else {
        fs::remove_file(&file).unwrap()
    }

    return Ok(());
}
