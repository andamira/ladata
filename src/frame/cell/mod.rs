// src/frame/cell
//
//! An individual data unit, categorized by data type and size.
//!
//

// enums with data, for cell data storage
mod data;
pub use data::{
    CategoricalData, CellData, CellDataNested, ContinuousData, DiscreteData, IdData, NumericalData,
};

mod able;
pub use able::CellAble;

// enums without data, for cell type information
mod r#type;
pub use r#type::{
    CategoricalType, CellType, CellTypeNested, ContinuousType, DiscreteType, IdType, NumericalType,
};
