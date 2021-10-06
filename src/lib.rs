mod defaults;
mod error;
mod configs;

use error::DotsyError;

use crate::defaults::create_default_dotsy_config;

pub type DotsyResult<T, E = DotsyError> = ::std::result::Result<T, E>;

#[derive(Debug)]
pub enum InitType {
    Config,
    Profile,
    Repo,
}

pub fn init(init_type: InitType) -> DotsyResult<()> {
    println!("{:?}", init_type);
    create_default_dotsy_config("./.dotsyrc.json").unwrap();
    Ok(())
}
