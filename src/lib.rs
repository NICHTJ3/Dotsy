mod error;
use error::DotsyError;

pub type DotsyResult<T, E = DotsyError> = ::std::result::Result<T, E>;
