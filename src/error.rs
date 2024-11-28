use snafu::Snafu;
use std::path::PathBuf;

#[derive(Snafu, Debug)]
pub enum DotsyError {
    #[snafu(display("I need an error message for this case"))]
    TODO,
    #[snafu(display("Failed to run shell command"))]
    FailedToRunCommand,
    #[snafu(display("Failed to unlink symlink {link}",link=link.display()))]
    Unlink { link: PathBuf },
    #[snafu(display("config was not found for {config_name}",config_name=config.display()))]
    ConfigNotAvailable { config: PathBuf },
    #[snafu(display("config file was not found"))]
    NoConfigFile,
    #[snafu(display(
        "we had some trouble linking files please check these paths: {from}, {to}",
        from=from.display(),
        to=to.display()
    ))]
    CouldntCreateSymLink { from: PathBuf, to: PathBuf },
    #[snafu(display(
        "we had some trouble creating the file or directory please check this path doesn't already exist: {file}",
        file=file.display()
    ))]
    FileAlreadyExists { file: PathBuf },
}
