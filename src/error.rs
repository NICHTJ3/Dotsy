use snafu::Snafu;
use std::path::PathBuf;

#[derive(Snafu, Debug)]
pub enum DotsyError {
    #[snafu(display("Failed to run shell command: {details}"))]
    FailedToRunCommand { details: String },
    #[snafu(display("Failed to unlink symlink at {link}: {reason}",link=link.display()))]
    Unlink { link: PathBuf, reason: String },
    #[snafu(display("Configuration file not found at {config_name}",config_name=config.display()))]
    ConfigNotAvailable { config: PathBuf },
    #[snafu(display("No configuration file found in expected locations"))]
    NoConfigFile,
    #[snafu(display(
        "Failed to create symlink from {from} to {to}: {reason}",
        from=from.display(),
        to=to.display()
    ))]
    CouldntCreateSymLink {
        from: PathBuf,
        to: PathBuf,
        reason: String,
    },
    #[snafu(display(
        "File or directory already exists at {file}",
        file=file.display()
    ))]
    FileAlreadyExists { file: PathBuf },
    #[snafu(display("I/O error: {details}"))]
    IoError { details: String },
    #[snafu(display("JSON parsing error: {details}"))]
    JsonError { details: String },
    #[snafu(display("Invalid configuration: {details}"))]
    InvalidConfig { details: String },
}
