use ansi_term::Color::Purple;
use ansi_term::Color::Yellow;
use std::{fs, os, path::PathBuf};

use crate::dotsy_log_error;
use crate::{
    configs::{DotsyConfig, Link},
    dotsy_err, dotsy_log_warning,
    error::DotsyError,
    get_absolute_link, is_symlink, link_exists, DotsyResult,
};

fn link(link: Link) -> DotsyResult<()> {
    let to = link.to;
    let from = link.from;
    println!(
        "{msg} {from} -> {to}",
        msg = Purple.paint("Attempting to link files:"),
        from = from.display(),
        to = to.display()
    );

    // A file, symlink, or directory alread exists at the location we're trying to link to
    if to.exists()
        && (to.is_file() || to.is_symlink() || (to.is_dir() && !link.glob.unwrap_or_default()))
    {
        return dotsy_err!(DotsyError::FileAlreadyExists { file: (to) });
    }

    // The file, or directory we're trying to link from doesn't exist
    if !from.is_file() && !from.is_dir() {
        return dotsy_err!(DotsyError::CouldntCreateSymLink {
            from: (from),
            to: (to)
        });
    }

    let to_dir = to.parent().unwrap();
    if !to_dir.exists() {
        fs::create_dir_all(to_dir).unwrap();
    };

    if os::unix::fs::symlink(&from, &to).is_err() {
        return dotsy_err!(DotsyError::CouldntCreateSymLink { from, to });
    }

    Ok(())
}

pub fn link_file(link_data: Link, global_config: &DotsyConfig) -> DotsyResult<()> {
    let link_data = get_absolute_link(link_data, global_config);
    if link_data.glob.unwrap_or_default() {
        let results = glob::glob(
            link_data
                .from
                .to_path_buf()
                .into_os_string()
                .to_str()
                .unwrap(),
        )
        .expect("Failed to glob files")
        .filter_map(Result::ok);

        let mut has_pre_existing_links = false;
        results.for_each(|file| {
            // We need to get the name of the subfile/dir to link to and create the path for the
            // alias on the fly
            // FIXME: The error handling should be handled by the caller
            let file_name = &file.file_name().unwrap();
            link(Link {
                from: file.to_path_buf(),
                to: link_data.to.join(file_name),
                glob: link_data.glob,
            })
            .unwrap_or_else(|e| match e {
                DotsyError::FileAlreadyExists { file: _ } => {
                    has_pre_existing_links = true;
                    println!("{}", Yellow.paint("Link already exists..."))
                }
                _ => {
                    dotsy_log_error!("{}", e);
                }
            });
        });
        if has_pre_existing_links {
            // TODO: Allow for debug flag that would log when the error occurs as well
            dotsy_log_warning!("Some links had existing files. You might want to consider uninstalling the config and re-installing");
        }
        Ok(())
    } else {
        link(link_data)
    }
}

// TODO: Fix this unlinking files that were not in the from path
pub fn unlink_file(link_data: Link, global_config: &DotsyConfig) -> DotsyResult<()> {
    let link_data = get_absolute_link(link_data, global_config);

    let file = link_data.to.to_path_buf();
    let should_glob = link_data.glob.unwrap_or_default();

    println!("Attempting to unlink: {}", &file.display());

    let files_to_unlink: Vec<PathBuf> = if should_glob {
        glob::glob(link_data.from.to_path_buf().to_str().unwrap())
            .expect("Failed to glob files")
            .filter_map(Result::ok)
            .map(|linked_file| file.join(linked_file.file_name().unwrap()))
            .collect()
    } else {
        if !link_exists(&file) {
            return dotsy_err!(DotsyError::Unlink { link: file });
        }
        vec![file.to_path_buf()]
    };

    files_to_unlink.iter().for_each(|file| {
        if !is_symlink(file) {
            return;
        }

        let file_type = fs::symlink_metadata(file).unwrap().file_type();

        println!("Unlinking {}", &file.display());

        if file_type.is_dir() {
            fs::remove_file(file).unwrap_or_else(|e| {
                dotsy_log_error!("{}", e);
            })
        } else {
            fs::remove_file(file).unwrap_or_else(|e| {
                dotsy_log_error!("{}", e);
            })
        }
    });
    println!("Done");

    Ok(())
}
