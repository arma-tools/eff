use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EddsError {
    #[error("IO failed")]
    EddsIOError(#[from] io::Error),

    #[error("Deku failed")]
    EddsDekuError(#[from] deku::DekuError),

    #[error("unknown decoding error")]
    Unknown,
}
