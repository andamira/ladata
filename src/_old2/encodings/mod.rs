//
//
//
//! Specific formatting of data in memory.
//!
//

mod api;
pub use api::*;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum DataEncoding<E: DataEncodingApi> {

    /// The classic 1 bit boolean.
    ///
    /// `Copy, Size:1, Align:1`
    Bool1,

    None,

    ///
    Other(E),
}

/// A zero-sized struct that facilitates using [`DataStructure`]
/// without custom structure types. See [`StandardDataStructure`].
#[non_exhaustive]
pub struct NoCustomDataEncodings;

// impl<S: DataStructureApi> DataStructureApi for NoCustomDataStructures {
impl DataEncodingApi for NoCustomDataEncodings {
    #[inline]
    fn is_serializer(&self) ->  bool { false }

    // FIXME :(
    // fn structure_type(&self) -> DataStructure<S> {
    //     DataStructure::None
    // }
}

/// [`DataEncoding`] without custom data types.
pub type StandardDataEncoding = DataEncoding<NoCustomDataEncodings>;

