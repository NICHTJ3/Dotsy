use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::{configs::DotsyConfig, DotsyResult};

pub fn create_default_dotsy_config(path: &str) -> DotsyResult<()> {
    let default_config = DotsyConfig::new(Path::new("~/Dotfiles").to_path_buf(), None, None);

    let serialized = serde_json::to_string_pretty(&default_config).unwrap();

    let mut file = File::create(path).unwrap();

    file.write_all(serialized.as_bytes()).unwrap();

    Ok(())
}
