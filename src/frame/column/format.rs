// src/frame/column/format
//
//!
//!
//

use crate::frame::cell::{CellData, CellDataNested};

/// Valid format types for inner storage.
pub enum FormatType {
    CellData,
    CellDataNested,
    Bytes,
}

/// Common abstraction for all inner format types.
pub trait Format {
    fn format_type(&self) -> FormatType;
}

impl Format for CellData {
    fn format_type(&self) -> FormatType {
        FormatType::CellData
    }
}

impl Format for CellDataNested {
    fn format_type(&self) -> FormatType {
        FormatType::CellDataNested
    }
}

impl Format for u8 {
    fn format_type(&self) -> FormatType {
        FormatType::Bytes
    }
}
