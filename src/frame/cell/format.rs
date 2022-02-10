// src/frame/cell/api
//
//!
//!
//

use super::{CellData, CellDataNested};

/// Common interface for all cell types
pub trait CellFormat {
    // WIP
}

impl CellFormat for CellData {}
impl CellFormat for CellDataNested {}
impl CellFormat for u8 {}
