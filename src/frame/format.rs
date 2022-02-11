// src/frame/format
//
//!
//!
//

use crate::frame::{CellData, Column, Row};

/*
/// List of types implementing [`Format`].
pub enum FormatType {
    /// Data is stored as a collection of [`CellData`]s.
    CellData,

    /// Data is stored as a collection of [`u8`]s.
    Bytes,
}
*/

// Implemented by data types used as *cell* storage by [`Column`]s and [`Row`]s.
/// Data types usable as inner *cell* storage format by [`Column`]s and [`Row`]s.
pub trait Format {
    // /// Returns the current [`FormatType`].
    // fn format_type(&self) -> FormatType;
}

impl Format for CellData {
    // fn format_type(&self) -> FormatType {
    //     FormatType::CellData
    // }
}

impl Format for u8 {
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
