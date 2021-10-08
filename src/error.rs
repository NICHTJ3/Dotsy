use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum DotsyError {
    #[snafu(display("error: I need an error message for this case"))]
    TODO,
    #[snafu(display("error: config file was not found"))]
    NoConfigFile,
    // TODO: Show the file names
    #[snafu(display(
        "error: we had some trouble linking theses files please check the paths {from}, {to}",
        from=from,
        to=to
    ))]
    CouldntCreateSymLink { from: String, to: String },
}
