// src/error
//
//!
//!

use core::result;

/// The result type for [`DataFrame`][crate::frame::DataFrame].
pub type Result<T> = result::Result<T, DataFrameError>;

/// The error type for [`DataFrame`][crate::frame::DataFrame].
#[derive(Debug, Clone)]
pub enum DataFrameError {
    Generic(String), // WIP TEMP
}

mod std_impls {
    use super::DataFrameError;
    use std::{error, fmt};

    impl fmt::Display for DataFrameError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            use DataFrameError::*;
            let s = match self {
                Generic(s) => format!["Generic DataFrameError: {}", s],
            };
            write!(f, "{}", &s)
        }
    }
    impl error::Error for DataFrameError {}
}
