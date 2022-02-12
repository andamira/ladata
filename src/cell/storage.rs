// ladata::cell::storage
//
//!
//!
//

// use crate::frame::{Column, Row};
use crate::cell::CellData;

/*
/// List of types implementing [`CellStorage`].
pub enum FormatType {
    /// Data is stored as a collection of [`CellData`]s.
    CellData,

    /// Data is stored as a collection of [`u8`]s.
    Bytes,
}
*/

/// Data types that can be used as internal *cell* storage by
/// [`Column`]s and [`Row`]s.
///
/// [`Column`]: crate::frame::Column
/// [`Row`]: crate::frame::Row
pub trait CellStorage {
    // /// Returns the current [`FormatType`].
    // fn format_type(&self) -> FormatType;
}

impl CellStorage for CellData {
    // fn format_type(&self) -> FormatType {
    //     FormatType::CellData
    // }
}

impl CellStorage for Option<CellData> {
}

impl CellStorage for u8 {
    // fn format_type(&self) -> FormatType {
    //     FormatType::Bytes
    // }
}

/*
macro_rules! impl_format {
    ($f:ty, $ftype:expr) => {
        impl Row<$f> {
            /// Returns the `FormatType` of this `Row`.
            pub const fn format_type(&self) -> FormatType {
                $ftype
            }
        }
        impl Column<$f> {
            /// Returns the `FormatType` of this `Column`.
            pub const fn format_type(&self) -> FormatType {
                $ftype
            }
        }
    };
}
impl_format![CellData, FormatType::CellData];
impl_format![u8, FormatType::Bytes];
*/
