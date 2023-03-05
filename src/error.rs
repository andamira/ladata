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

    /// The given index is out of bounds.
    // /// The given index in row or column major order was out of bounds.
    IndexOutOfBounds(usize),

    /// The given indices 2d are out of bounds.
    // TODO: more dimensions
    Indices2dOutOfBounds(usize, usize),

    /// The given indices 2d were out of bounds for a chunk of the given length.
    ChunkIndices2dOutOfBounds(usize, usize, usize),

    ///
    Overflow,

    ///
    Underflow,

    /// The dimensions given did not match the elements provided
    DimensionMismatch,

    /// The node is empty.
    EmptyNode,
}

/// impl Display & Error
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod std_impls {
    use super::LadataError;
    use std::{error::Error as StdError, fmt};

    impl StdError for LadataError {}

    impl fmt::Display for LadataError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
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
                LadataError::IndexOutOfBounds(i) => write!(f, "Index {i} is out of bounds."),
                LadataError::Indices2dOutOfBounds(i, j) => {
                    write!(f, "Indices 2d: {i}, {j} are out of bounds.")
                }
                LadataError::ChunkIndices2dOutOfBounds(i, j, k) => write!(
                    f,
                    "Indices 2d {i}, {j} are out of bounds for a chunk of length {k}."
                ),
                LadataError::Overflow => write!(f, "Overflow."),
                LadataError::Underflow => write!(f, "Underflow."),

                LadataError::DimensionMismatch => write!(f, "Dimension Mismatch."),
                LadataError::EmptyNode => write!(f, "The node is empty."),
            }
        }
    }
}
