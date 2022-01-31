// src/dataframe/cell
//! A cell in a `DataFrame`, part of a `Column` and/or a `Row`.
//!

use uuid::Uuid;

use crate::dataframe::handle::{Handle128, Handle16, Handle32, Handle64, Handle8};

mod data;
mod impls;
mod r#type;

pub use data::{
    CategoricalData, CellData, CellDataNested, ContinuousData, DiscreteData, IdData, NumericalData,
};
pub use r#type::{
    CategoricalType, CellType, CellTypeNested, ContinuousType, DiscreteType, IdType, NumericalType,
};

/// A trait that all inner underlying cell data types must implement.
pub trait InnerCellDataType {
    /// Returns the [`CellType`] variant that can contain this type of data.
    fn cell_type(&self) -> CellType;

    /// Returns the [`CellTypeNested`] variant that can contain this type of data.
    fn cell_type_nested(&self) -> CellTypeNested {
        self.cell_type().into()
    }
}

macro_rules! inner_impl_cell_data_type {
    ($type:ty, $cell_type:expr) => {
        impl InnerCellDataType for $type {
            fn cell_type(&self) -> CellType {
                $cell_type
            }
        }
    };
}
// Categorical
inner_impl_cell_data_type![bool, CellType::Bool];
inner_impl_cell_data_type![String, CellType::String];
inner_impl_cell_data_type![Vec<u8>, CellType::Bytes];
inner_impl_cell_data_type![Handle8, CellType::Handle8];
inner_impl_cell_data_type![Handle16, CellType::Handle16];
inner_impl_cell_data_type![Handle32, CellType::Handle32];
inner_impl_cell_data_type![Handle64, CellType::Handle64];
inner_impl_cell_data_type![Handle128, CellType::Handle128];
inner_impl_cell_data_type![Uuid, CellType::Uuid];
// Numerical
inner_impl_cell_data_type![f32, CellType::F32];
inner_impl_cell_data_type![f64, CellType::F64];
inner_impl_cell_data_type![i8, CellType::I8];
inner_impl_cell_data_type![u8, CellType::U8];
inner_impl_cell_data_type![i16, CellType::I16];
inner_impl_cell_data_type![u16, CellType::U16];
inner_impl_cell_data_type![i32, CellType::I32];
inner_impl_cell_data_type![u32, CellType::U32];
inner_impl_cell_data_type![i64, CellType::I64];
inner_impl_cell_data_type![u64, CellType::U64];
inner_impl_cell_data_type![i128, CellType::I128];
inner_impl_cell_data_type![u128, CellType::U128];
