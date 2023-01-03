// ladata::error
//
//! Error types.
//

use core::result;

/// `ladata` result type.
pub type LadataResult<N> = result::Result<N, LadataError>;

/// `ladata` error type.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LadataError {
    /// The index is out of bounds.
    IndexOutOfBounds,

    /// There are not enough elements for the operation.
    ///
    /// Contains the minimum number of elements needed.
    NotEnoughElements(usize),

    /// There is not enough free space for the operation.
    ///
    /// Optionally contains the number of free spaces needed.
    NotEnoughSpace(Option<usize>),

    /// The key already exists.
    KeyAlreadyExists,
}

/// impl Display & Error
#[cfg(feature = "std")]
mod std_impls {
    use super::LadataError;
    use std::{error::Error as StdError, fmt};

    impl StdError for LadataError {}

    impl fmt::Display for LadataError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LadataError::IndexOutOfBounds => write!(f, "Index is out of bounds."),
                LadataError::NotEnoughElements(n) => {
                    write!(f, "Not enough elements. Needs at least `{n}` elements.")
                }
                LadataError::NotEnoughSpace(n) => {
                    if let Some(n) = n {
                        write!(
                            f,
                            "Not enough space. Needs at least `{n}` free space for elements."
                        )
                    } else {
                        write!(f, "Not enough space.")
                    }
                }
                LadataError::KeyAlreadyExists => write!(f, "The key already exists."),
            }
        }
    }
}
