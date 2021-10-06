use snafu::Snafu;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum DotsyError {
    #[snafu(display("error: something went wrong"))]
    ExampleError,
}
