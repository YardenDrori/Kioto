use std::io::Error;

use crate::error::KiotoError;

pub enum ReactorError {
    IoError(Error),
}

impl From<ReactorError> for KiotoError {
    fn from(value: ReactorError) -> Self {
        Self::ReactorError(value)
    }
}
