pub mod generate;
pub mod init;
pub mod utils;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("failed to convert OsString to String: {0}")]
    OsStringToStringError(String),
}
