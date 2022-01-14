use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum DotsyError {
    #[snafu(display("Error: I need an error message for this case"))]
    TODO,
    #[snafu(display("Error: config file was not found"))]
    NoConfigFile,
    #[snafu(display(
        "Error: we had some trouble linking files please check these paths: {from}, {to}",
        from=from,
        to=to
    ))]
    CouldntCreateSymLink { from: String, to: String },
    #[snafu(display(
        "Error: we had some trouble the file or directory please check this path doesn't already exist: {file}",
        file=file
    ))]
    FileAlreadyExists { file: String },
}
