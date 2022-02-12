// ladata::error
// pub
//
//! Data error types.
//!
//

use core::result;

/// The result type for [`DataFrame`][crate::frame::DataFrame].
pub type DataResult<T> = result::Result<T, DataError>;

/// The error type for [`DataFrame`][crate::frame::DataFrame].
#[derive(Debug, Clone)]
pub enum DataError {
    Generic(String), // WIP TEMP
}

mod std_impls {
    use super::DataError;
    use std::{error, fmt};

    impl fmt::Display for DataError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            use DataError::*;
            let s = match self {
                Generic(s) => format!["Generic DataError: {}", s],
            };
            write!(f, "{}", &s)
        }
    }
    impl error::Error for DataError {}
}
