// src/types/variants
//
//
//!
//!
//

use super::{DataTypePlus, DataTypes, NoDataType};
// use super::{DataType, DataTypes, WithoutCustomDataTypes};

/// [`DataType`] without custom data types.
// pub type DataTypeSimple = DataType<NoCustomDataType>;
// pub type DataTypeSimple = DataType<WithoutCustomDataTypes>;
// pub type DataTypeSimple = DataType<NoDataType>;
//
// pub type DataType = DataTypePlus<NoDataType>; // …






// MAYBE?
// /// [`DataType`] without custom data.
// pub type Dt = DataType<NoCustomTypes>;


// IDEAS
// - MAYBE: Differentiate between Copy & NonCopy data types …
// - mainly just `CopyDataType` is useful, for static checking.

// ///
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub enum CopyDataType {
//     Custom(T),
// }
//
// ///
// #[derive(Clone, Debug, PartialEq)]
// pub enum NonCopyDataType {
// }


// impl<T: DataTypeApi> DataTypeApi for CopyDataType<T> {
//     fn is_copy(&self) -> bool {
//         true
//     }
// }
//
// impl<T: DataTypeApi> DataTypeApi for NonCopyDataType<T> {
//     fn is_copy(&self) -> bool {
//         false
//     }
// }


/// EXAMPLE
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MediaDataType {
    ///
    Text,
    ///
    TextJson,
    ///
    TextXml,
    ///
    TextCsv,

    ///
    Image,
    ///
    ImagePng,
    ///
    ImageJpg,
    ///
    ImageGif,

    ///
    Audio,
    ///
    AudioMp3,


    ///
    Video,
    ///
    VideoMatroska,
    ///
    VideoAvi,
    ///
    VideoMpeg,
}

// RETHINK: to make it embeddable, impl From<>?
// - simplify embedding…
//   - removing generic <T> from trait ← most beautiful
//   - (how to return?)
//   - 
// impl DataTypes for MediaDataType {
//     #[inline]
//     fn is_copy(&self) -> bool { false }
//
//     // RETHINK: either need From<> or >> use it to impl From automatically
//     fn to_data_type<T: DataTypes>(&self) -> DataType<T>{
//         DataType::Other(self) // FIXME: too recursive?
//     }
// }
