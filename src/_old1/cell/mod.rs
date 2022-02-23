// ladata::cell
// pub
//
//! Data units.
//!
//! Abstractions over minimal practical data units that can be used as a
//! building block by more complex data structures.
//!
//

mod able;
pub use able::CellAble;

mod data;
pub use data::CellData;

mod impls;

mod storage;
pub use storage::CellStorage;

mod r#type;
pub use r#type::CellType;

/// Nested versions of [`CellData`] and [`CellType`].
pub mod nested {
    pub use super::data::{
        CategoricalData, CellData, CellDataNested, ContinuousData, DiscreteData, IdData,
        NumericalData,
    };
    pub use super::r#type::{
        CategoricalType, CellType, CellTypeNested, ContinuousType, DiscreteType, IdType,
        NumericalType,
    };
}

/// Represents the absence of a cell.
pub struct NoCell;

// pub struct Cell<S: CellStorage> {
// }
