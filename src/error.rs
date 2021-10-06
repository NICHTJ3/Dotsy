use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum DotsyError {
    #[snafu(display("error: config file was not found"))]
    NoConfigFile,
}
