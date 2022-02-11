// src/frame/cell
//
//! A cell in a `DataFrame`, part of a `Column` and/or a `Row`.
//!
//

// enums with data, for cell data storage
mod data;
pub use data::{
    CategoricalData, CellData, CellDataNested, ContinuousData, DiscreteData, IdData, NumericalData,
};

mod acceptable;
pub use acceptable::AcceptableData;

// enums without data, for cell type information
mod r#type;
pub use r#type::{
    CategoricalType, CellType, CellTypeNested, ContinuousType, DiscreteType, IdType, NumericalType,
};