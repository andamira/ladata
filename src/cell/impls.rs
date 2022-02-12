// ladata::cell::std_impls
//
//!
//!
//

use super::{CellData, CellType, nested::{CellDataNested, CellTypeNested}};

impl Default for CellType {
    fn default() -> Self {
        Self::None
    }
}
impl Default for CellTypeNested {
    fn default() -> Self {
        Self::None
    }
}

impl Default for CellData {
    fn default() -> Self {
        Self::None
    }
}
impl Default for CellDataNested {
    fn default() -> Self {
        Self::None
    }
}
