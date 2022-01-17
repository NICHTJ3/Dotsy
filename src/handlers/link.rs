use std::{fs, os, path::PathBuf};

use crate::{
    configs::{DotsyConfig, Link},
    dotsy_err,
    error::DotsyError,
    get_absolute_link, is_symlink, link_exists, DotsyResult,
};

fn link(link: Link) -> DotsyResult<()> {
    let to = link.to;
    let from = link.from;
    println!("linking {} -> {}", from.display(), to.display());

    if to.exists() && (to.is_file() || to.is_dir() && !link.glob.unwrap_or_default())
        || !from.is_file() && !from.is_dir()
    {
        dotsy_err!(DotsyError::CouldntCreateSymLink { from: from, to: to });
    }

    let to_dir = to.parent().unwrap();
    if !to_dir.exists() {
        fs::create_dir_all(to_dir).unwrap();
    };

    if let Err(..) = os::unix::fs::symlink(&from, &to) {
        dotsy_err!(DotsyError::CouldntCreateSymLink { from: from, to: to });
    }

    return Ok(());
}

pub fn link_file(link_data: Link, global_config: &DotsyConfig) -> DotsyResult<()> {
    let link_data = get_absolute_link(link_data, global_config);
    if link_data.glob.unwrap_or_default() {
        let results = glob::glob(
            &link_data
                .from
                .to_path_buf()
                .into_os_string()
                .to_str()
                .unwrap()
                .to_string(),
        )
        .expect("Failed to glob files")
        .filter_map(Result::ok);
        results.for_each(|file| {
            // We need to get the name of the subfile/dir to link to and create the path for the
            // alias on the fly
            // FIXME: This Should be handled by the caller
            let file_name = &file.file_name().unwrap();
            link(Link {
                from: file.to_path_buf(),
                to: link_data.to.join(&file_name),
                glob: link_data.glob,
            })
            .unwrap_or_else(|e| {
                eprintln!("{}", e);
            });
        });
        Ok(())
    } else {
        link(link_data)
    }
}

pub fn unlink_file(link_data: Link, global_config: &DotsyConfig) -> DotsyResult<()> {
    let link_data = get_absolute_link(link_data, global_config);

    let file = link_data.to.to_path_buf();
    let should_glob = link_data.glob.unwrap_or_default();

    println!("Attempting to unlink: {}", &file.display());

    let files_to_unlink: Vec<PathBuf> = if should_glob {
        let file = file.join("*");
        let pattern = file.to_str().unwrap();
        glob::glob(&pattern)
            .expect("Failed to glob files")
            .filter_map(Result::ok)
            .collect()
    } else {
        if !link_exists(&file) {
            dotsy_err!(DotsyError::Unlink { link: file });
        }
        vec![file.to_path_buf()]
    };

    files_to_unlink.into_iter().for_each(|file| {
        if !is_symlink(&file) {
            return;
        }

        let file_type = fs::symlink_metadata(&file).unwrap().file_type();

        println!("Unlinking {}", &file.display());

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
    println!("Done");

    return Ok(());
}
