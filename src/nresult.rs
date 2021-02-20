use std::option::NoneError;

use bindings::{
    windows::Error
};

#[derive(Debug)]
pub enum NError {
    Rt(Error),
    Null(NoneError),
}

impl From<Error> for NError {
    #[inline]
    fn from(e: Error) -> Self {
        NError::Rt(e)
    }
}

impl From<NoneError> for NError {
    #[inline]
    fn from(n: NoneError) -> Self {
        NError::Null(n)
    }
}

pub type NResult<T> = std::result::Result<T, NError>;
