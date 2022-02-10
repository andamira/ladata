// src/frame/cell/inner
//
//!
//!
//

use crate::frame::{
    cell::{CellData, CellDataNested, CellType, CellTypeNested},
    handle::{Handle128, Handle16, Handle32, Handle64, Handle8},
};

/// A trait that all inner underlying cell data types must implement.
pub trait AcceptableData {
    /// Returns the [`CellType`] variant that can contain this type of data.
    fn cell_type(&self) -> CellType;

    /// Returns the [`CellTypeNested`] variant that can contain this type of data.
    fn cell_type_nested(&self) -> CellTypeNested {
        self.cell_type().into()
    }

    fn to_cell_data(&self) -> CellData;
    fn into_cell_data(self) -> CellData;
    fn to_cell_data_nested(&self) -> CellDataNested {
        self.to_cell_data().into()
    }

    fn into_cell_data_nested(self) -> CellDataNested
    where
        Self: Sized,
    {
        self.into_cell_data().into()
    }

}

//
macro_rules! inner_impl_cell_data_type {
    ($type:ty, $cell_type:expr, $cell_data:expr) => {
        inner_impl_cell_data_type![owned; $type, $cell_type, $cell_data];
        inner_impl_cell_data_type![refs; &$type, $cell_type, $cell_data];
        inner_impl_cell_data_type![refs; &mut $type, $cell_type, $cell_data];
    };

    (owned; $type:ty, $cell_type:expr, $cell_data:expr) => {
        impl AcceptableData for $type {
            fn cell_type(&self) -> CellType {
                $cell_type
            }

            fn to_cell_data(&self) -> CellData {
                $cell_data(*self)
            }

            fn into_cell_data(self) -> CellData {
                $cell_data(self)
            }
        }
    };
    (refs; $type:ty, $cell_type:expr, $cell_data:expr) => {
        impl AcceptableData for $type {
            fn cell_type(&self) -> CellType {
                $cell_type
            }

            fn to_cell_data(&self) -> CellData {
                $cell_data(**self)
            }

            fn into_cell_data(self) -> CellData {
                $cell_data(*self)
            }
        }
    };
}

// Categorical

inner_impl_cell_data_type![bool, CellType::Bool, CellData::Bool];
// inner_impl_cell_data_type![CStrPtr<'_>, CellType::String];
// inner_impl_cell_data_type![Vec<u8>, CellType::Bytes];

// inner_impl_cell_data_type![Uuid, CellType::Uuid];
inner_impl_cell_data_type![Handle8, CellType::Handle8, CellData::Handle8];
inner_impl_cell_data_type![Handle16, CellType::Handle16, CellData::Handle16];
inner_impl_cell_data_type![Handle32, CellType::Handle32, CellData::Handle32];
inner_impl_cell_data_type![Handle64, CellType::Handle64, CellData::Handle64];
inner_impl_cell_data_type![Handle128, CellType::Handle128, CellData::Handle128];

// Numerical
inner_impl_cell_data_type![f32, CellType::F32, CellData::F32];
inner_impl_cell_data_type![f64, CellType::F64, CellData::F64];
inner_impl_cell_data_type![i8, CellType::I8, CellData::I8];
inner_impl_cell_data_type![u8, CellType::U8, CellData::U8];
inner_impl_cell_data_type![i16, CellType::I16, CellData::I16];
inner_impl_cell_data_type![u16, CellType::U16, CellData::U16];
inner_impl_cell_data_type![i32, CellType::I32, CellData::I32];
inner_impl_cell_data_type![u32, CellType::U32, CellData::U32];
inner_impl_cell_data_type![i64, CellType::I64, CellData::I64];
inner_impl_cell_data_type![u64, CellType::U64, CellData::U64];
inner_impl_cell_data_type![i128, CellType::I128, CellData::I128];
inner_impl_cell_data_type![u128, CellType::U128, CellData::U128];

// impl AcceptableData for bool {
//     // fn needs_conversion(&self) -> bool { true }
//
//     fn cell_type(&self) -> CellType {
//         CellType::Bool
//     }
// }
