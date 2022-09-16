use ansi_term::Colour::Red;
use snafu::Snafu;
use std::path::PathBuf;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum DotsyError {
    #[snafu(display("{}: I need an error message for this case", Red.bold().paint("Error")))]
    TODO,
    #[snafu(display("{}: Failed to run shell command",Red.bold().paint("Error")))]
    FailedToRunCommand,
    #[snafu(display("{error}: Failed to unlink symlink {link}",link=link.display(),error=Red.bold().paint("Error")))]
    Unlink { link: PathBuf },
    #[snafu(display("{error}: config file was not found",error=Red.bold().paint("Error")))]
    NoConfigFile,
    #[snafu(display(
        "{error}: we had some trouble linking files please check these paths: {from}, {to}",
        error=Red.bold().paint("Error"),
        from=from.display(),
        to=to.display()
    ))]
    CouldntCreateSymLink { from: PathBuf, to: PathBuf },
    #[snafu(display(
        "{error}: we had some trouble creating the file or directory please check this path doesn't already exist: {file}",
        error=Red.bold().paint("Error"),
        file=file.display()
    ))]
    FileAlreadyExists { file: PathBuf },
}
