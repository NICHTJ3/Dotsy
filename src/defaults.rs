use std::path::PathBuf;

use crate::{dotsy_err, error::DotsyError, home, xdg_config_home, DotsyResult};

pub fn fallback_path() -> DotsyResult<PathBuf> {
    let default_config_paths: Vec<Option<PathBuf>> = vec![
        Some(PathBuf::from("./.dotsyrc.json")),
        xdg_config_home!("dotsy/dotsyrc.json"),
        xdg_config_home!("dotsy/dotsyrc"),
        xdg_config_home!("dotsyrc.json"),
        home!(".dotsyrc.json"),
    ];

    // Loops for a vector of possible paths and tries to generate config from the first
    // default path that exists.
    for config_path in default_config_paths {
        if let Some(path) = config_path {
            if path.exists() {
                return Ok(path);
            }
        }
    }
    dotsy_err!(DotsyError::NoConfigFile)
}
