// ladata::cell
// pub
//
//! Abstrations over individual data units.
//!
//! Mainly categorized by data type and size.
//!
//

// enums with data, for cell data storage
mod data;
pub use data::CellData;

mod able;
pub use able::CellAble;

// enums without data, for cell type information
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
