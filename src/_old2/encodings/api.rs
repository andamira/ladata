// src/encoding/api
//
//
//!
//!
//


pub use super::DataEncoding;

/// All supported data encodings must implement this trait.
///
/// ## Example
// ```
// ```
pub trait DataEncodingApi {
    /// Returns `true` if the current encoding serializes data.
    fn is_serializer(&self) -> bool;

}

