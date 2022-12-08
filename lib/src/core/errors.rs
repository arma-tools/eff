use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EddsError {
    #[error("IO failed")]
    EddsIOError(#[from] io::Error),

    #[error("Deku failed")]
    EddsDekuError(#[from] deku::DekuError),

    #[error("bcndecode error")]
    BcndecodeError(#[from] bcndecode::Error),

    #[error("Unknown image data format: `{0}`!\nPlease report this error at https://github.com/aff-org/eff/issues")]
    UnknownImageDataFormat(String),

    #[error("Unknown image data type: `{0}`!\nPlease report this error at https://github.com/aff-org/eff/issues")]
    UnknownImageDataType(String),

    #[error("unknown decoding error")]
    Unknown,
}
