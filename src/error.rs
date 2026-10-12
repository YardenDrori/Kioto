use crate::reactor::error::ReactorError;

pub enum KiotoError {
    ReactorError(ReactorError),
}
