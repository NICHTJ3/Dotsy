use std::{fs, os, path::PathBuf};

use crate::{dotsy_err, dotsy_warn, error::DotsyError, DotsyResult};

fn link(from: &PathBuf, to: &PathBuf, should_glob: bool) -> DotsyResult<()> {
    let from_path_as_string = from.to_string_lossy().to_string();
    let to_path_as_string = to.to_string_lossy().to_string();
    println!("linking {} -> {}", from_path_as_string, to_path_as_string);

    if to.exists() && (to.is_file() || to.is_dir() && !should_glob)
        || !from.is_file() && !from.is_dir()
    {
        dotsy_err!(DotsyError::CouldntCreateSymLink {
            from: from_path_as_string,
            to: to_path_as_string,
        });
    }

    let to_dir = to.parent().unwrap();
    if !to_dir.exists() {
        fs::create_dir_all(to_dir).unwrap();
    };

    if let Err(..) = os::unix::fs::symlink(from, to) {
        // TODO: This should log as we go?
        dotsy_warn!("Something went wrong symlinking");
        dotsy_err!(DotsyError::CouldntCreateSymLink {
            from: from_path_as_string,
            to: to_path_as_string
        });
    }

    return Ok(());
}

// TODO: Rethink how this is passed should this actually receive the whole link object?
pub fn link_file(from: PathBuf, to: PathBuf, should_glob: bool) -> DotsyResult<()> {
    if should_glob {
        let mut results = glob::glob(&from.into_os_string().to_str().unwrap().to_string())
            .expect("Failed to glob files")
            .into_iter()
            .filter_map(Result::ok);
        // Skip the first item since it appears to be the glob pattern
        results.next();
        results.for_each(|file| {
            // We need to get the name of the subfile/dir to link to and create the path for the
            // alias on the fly
            // FIXME: This Should be handled by the caller
            let file_name = &file.file_name().unwrap();
            link(&file, &to.join(file_name), should_glob).unwrap_or_else(|e| {
                eprintln!("{}", e);
            });
        });
        Ok(())
    } else {
        link(&from, &to, should_glob)
    }
}

// TODO: This should handle globs
pub fn unlink_file<'a>(file: &'a PathBuf, should_glob: bool) -> DotsyResult<()> {
    let file_path = if should_glob {
        let file = file.join("*");
        file
    } else {
        file.to_path_buf()
    };

    let pattern = file_path.to_str().unwrap();
    let mut files_to_unlink: Vec<PathBuf> = glob::glob(&pattern)
        .expect("Failed to glob files")
        .filter_map(Result::ok)
        .collect();
    if !should_glob {
        files_to_unlink.push(file.to_path_buf());
    }

    files_to_unlink.into_iter().for_each(|file| {
        if !&file.exists() {
            eprintln!(
                "{}",
                DotsyError::Unlink {
                    link: file.to_str().unwrap().to_string()
                }
            );
            return;
        }
        let metadata = fs::symlink_metadata(&file).unwrap();
        let file_type = metadata.file_type();

        let is_symlink = file_type.is_symlink();
        if !is_symlink {
            // FIXME: This should be handled at an upper level
            eprintln!(
                "{}",
                DotsyError::Unlink {
                    link: file.to_str().unwrap().to_string()
                }
            );
            return;
        }

        if file_type.is_dir() {
            fs::remove_file(&file).unwrap_or_else(|e| {
                eprintln!("{}", e);
            })
        } else {
            fs::remove_file(&file).unwrap_or_else(|e| {
                eprintln!("{}", e);
            })
        }
    });

    return Ok(());
}
